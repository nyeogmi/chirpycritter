# NYEO NOTE: I couldn't find any good examples of _exactly_ what old synth waveforms looked like, so let's be creative
def table(fn):
    return [fn(x / 1024.0) for x in range(1024)]

# sine wave
def sine_wave(x):
    from math import sin
    return sin(x * 2 * pi)

# square wave
def square_wave(x):
    if x < 0.5:
        basis = -square_peak(x)
    else:
        basis = square_peak(x)
    if x in [0.45, 0.55]:
        basis += sin(x * 8)
    return basis

def square_peak(x):
    return 0.6 + 0.4 * x * (0.8 + sin(x * 4) * (0.2 * (1.0 - x**2)))

# triangle wave
def triangle_wave(x):
    if x <= 0.5:
        base = 2 * triangle_slant(x / 0.5) - 1
    else:
        base = -2 * triangle_slant((x - 0.5) / 0.5) + 1

    dir = -1 if base < 0 else 1
    mag = abs(base)
    return dir * mag ** 0.8

def triangle_slant(x):
    return x ** 1.2

# sawtooth wave
def sawtooth_wave(x):
    base = (2 * (x ** 0.8) - 1 - 0.12) * 0.88
    dir = -1 if base < 0 else 1
    mag = abs(base)
    return dir * mag ** 0.6

with open("src/math/tables.rs", "wt") as f:
    from math import sin, pi
    for name, fn in [("SIN", sine_wave), ("SQUARE", square_wave), ("TRIANGLE", triangle_wave), ("SAW", sawtooth_wave)]:
        tab = table(fn)
        print("Mean of {}: {} ({}){}".format(name, sum(tab)/len(tab), sum(map(abs, tab))/len(tab), " (clips)" if any(abs(t) > 1.0 for t in tab) else ""))
        f.write("pub const {}: [f32; 1024] = {};\n".format(name, table(fn)))