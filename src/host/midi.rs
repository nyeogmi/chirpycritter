use std::{borrow::Cow, collections::{BTreeMap, btree_map::Entry}};

use midly::num::{u4, u7};

use crate::song::*;

pub fn convert_midi(bytes: &[u8]) -> Song {
    let smf = midly::Smf::parse(bytes).unwrap();
    let mut all_notes: BTreeMap<u64, Vec<Packet>> = BTreeMap::new();

    let ticks_per_beat = match smf.header.timing {
        midly::Timing::Metrical(ticks_per_beat) => {
            ticks_per_beat.as_int() as u32
        }
        midly::Timing::Timecode(_, _) => todo!(),  // TODO: See if we get a file lke this
    };

    let mut microseconds_per_beat = 500000;  // 120 BPM

    let mut channel_program: [u7; 16] = [u7::from(0); 16];  // TODO: Understand program changes _during_ a song
    for track in smf.tracks.iter() {
        for &evt in track.iter() {
            match evt.kind {
                midly::TrackEventKind::Midi { channel, message: midly::MidiMessage::ProgramChange { program }} => {
                    channel_program[channel.as_int() as usize] = program;
                }
                _ => {}
            }
        }
    }

    for track in smf.tracks {
        struct NoteOn {
            start: u64,
            channel: u4,
        }

        let mut tick: u64 = 0;
        // TODO: Velocity and stuff too
        let mut notes_on: BTreeMap<u7, NoteOn> = BTreeMap::new();
        // NOTE: For now, deliberately misinterpret programs as one program per track instead of one program per channel

        for &evt in track.iter() {
            tick += evt.delta.as_int() as u64;

            match evt.kind {
                midly::TrackEventKind::Midi { channel, message } => {
                    match message {
                        midly::MidiMessage::NoteOn { key, vel } if vel > 0 => {
                            notes_on.insert(key, NoteOn {
                                start: tick,
                                channel,
                            });
                        }
                        midly::MidiMessage::NoteOff { key, vel: _ } | midly::MidiMessage::NoteOn { key, vel: _ } => {
                            if let Some(note_on) = notes_on.remove(&key) {
                                // TODO: as_int -- actually convert to hertz!!!
                                let packet = Packet::Play {
                                    channel: note_on.channel.as_int() as u16,
                                    // program: channel_program[note_on.channel.as_int() as usize].as_int() as u16,
                                    frequency: to_hertz(key), 
                                    duration: (tick - note_on.start) as u16,
                                };
                                match all_notes.entry(note_on.start) {
                                    Entry::Occupied(mut o) => { o.get_mut().push(packet); }
                                    Entry::Vacant(v) => { v.insert(vec![packet]); }
                                }
                            }
                        }
                        _  => {}
                    }
                }

                midly::TrackEventKind::Meta(midly::MetaMessage::Tempo(tempo)) => {
                    microseconds_per_beat = tempo.as_int();
                }

                _ => {}
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
        if tick > last_tick { 
            if song_packets.len() > 0 {
                song_packets.push(Packet::Wait((tick - last_tick) as u16))
            }
        }
        last_tick = tick;

        for packet in packets {
            song_packets.push(packet)
        }
    };
    return Song { 
        ticks_per_second: ticks_per_second as u64,
        ticks_per_beat: ticks_per_beat as u64,
        data: Cow::Owned(song_packets),
    }
}

fn to_hertz(key: u7) -> u16 {
    let key = key.as_int() as u16;

    return (((2.0f32).powf((key as f32 - 69.0) / 12.0) * 440.0).min(u16::MAX as f32).max(u16::MIN as f32)) as u16
}