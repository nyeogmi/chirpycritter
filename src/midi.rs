use std::{borrow::Cow, collections::{BTreeMap, btree_map::Entry}};

use midly::num::u7;

use crate::{Packet, Song};

pub fn convert_midi(bytes: &[u8]) -> Song {
    let smf = midly::Smf::parse(bytes).unwrap();
    let mut all_notes: BTreeMap<u64, Vec<Packet>> = BTreeMap::new();

    let mut ticks_per_beat = match smf.header.timing {
        midly::Timing::Metrical(ticks_per_beat) => {
            ticks_per_beat.as_int() as u32
        }
        midly::Timing::Timecode(_, _) => todo!(),  // TODO: See if we get a file lke this
    };

    let mut microseconds_per_beat = 500000;  // 120 BPM

    for track in smf.tracks {
        let mut tick: u64 = 0;
        // TODO: Velocity and stuff too
        let mut notes_on: BTreeMap<u7, u64> = BTreeMap::new();

        for &evt in track.iter() {
            tick += evt.delta.as_int() as u64;

            match evt.kind {
                midly::TrackEventKind::Midi { channel, message } => {
                    let chan = channel.as_int();  // TODO: Use this info

                    match message {
                        midly::MidiMessage::NoteOn { key, vel } if vel > 0 => {
                            notes_on.insert(key, tick);
                        }
                        midly::MidiMessage::NoteOff { key, vel } | midly::MidiMessage::NoteOn { key, vel } =>  {
                            if let Some(start) = notes_on.remove(&key) {
                                // TODO: as_int -- actually convert to hertz!!!
                                let packet = Packet::Play(to_hertz(key), (tick - start) as u16);
                                match all_notes.entry(start) {
                                    Entry::Occupied(mut o) => { o.get_mut().push(packet); }
                                    Entry::Vacant(v) => { v.insert(vec![packet]); }
                                }
                            }
                        }
                        midly::MidiMessage::ProgramChange { program } => {
                            // NYEO NOTE: This one matters!
                        }
                        midly::MidiMessage::Aftertouch { .. } => { }
                        midly::MidiMessage::Controller { .. } => { },
                        midly::MidiMessage::ChannelAftertouch { .. } => { },
                        midly::MidiMessage::PitchBend { .. } => { },
                    }
                }

                // TODO: Catch these?
                midly::TrackEventKind::SysEx(_) => {}
                midly::TrackEventKind::Escape(_) => {}
                midly::TrackEventKind::Meta(m) => {
                    match m {
                        midly::MetaMessage::TrackNumber(_) => {}
                        midly::MetaMessage::Text(_) => {}
                        midly::MetaMessage::Copyright(_) => {}
                        midly::MetaMessage::TrackName(_) => {}
                        midly::MetaMessage::InstrumentName(_) => {}
                        midly::MetaMessage::Lyric(_) => {}
                        midly::MetaMessage::Marker(_) => {}
                        midly::MetaMessage::CuePoint(_) => {}
                        midly::MetaMessage::ProgramName(_) => {}
                        midly::MetaMessage::DeviceName(_) => {}
                        midly::MetaMessage::MidiChannel(_) => {}
                        midly::MetaMessage::MidiPort(_) => {}
                        midly::MetaMessage::EndOfTrack => {}
                        midly::MetaMessage::Tempo(tempo) => {
                            microseconds_per_beat = tempo.as_int();
                        }
                        midly::MetaMessage::SmpteOffset(_) => {}
                        midly::MetaMessage::TimeSignature(_, _, _, _) => {}
                        midly::MetaMessage::KeySignature(_, _) => {}
                        midly::MetaMessage::SequencerSpecific(_) => {}
                        midly::MetaMessage::Unknown(_, _) => {}
                    }
                }
            }
        }
    }

    songify(ticks_per_beat, microseconds_per_beat, all_notes)
}

fn songify(ticks_per_beat: u32, microseconds_per_beat: u32, all_notes: BTreeMap<u64, Vec<Packet>>) -> Song {
    // TODO: Do in floating point?
    let beats_per_second = 1000000 / microseconds_per_beat;
    let ticks_per_second = ticks_per_beat * beats_per_second;

    // TODO: Calculate a "tick divisor" based on greatest common denominator of all tick timings

    let mut song_packets = Vec::new();

    let mut last_tick: u64 = 0;
    for (tick, packets) in all_notes {
        if tick > last_tick { song_packets.push(Packet::Wait((tick - last_tick) as u16)) }
        last_tick = tick;

        for packet in packets {
            song_packets.push(packet)
        }
    };
    return Song { 
        ticks_per_second: ticks_per_second as u64,
        data: Cow::Owned(song_packets),
    }
}

fn to_hertz(key: u7) -> u16 {
    let key = key.as_int() as u16;

    return (((2.0f32).powf((key as f32 - 69.0) / 12.0) * 440.0).min(u16::MAX as f32).max(u16::MIN as f32)) as u16
}