# CLS - Command Line Synthesizer

CLS is a command-line tool for generating audio waveforms with envelope control. It supports basic waveforms and ADSR envelope shaping.

## Features

- Basic waveform generation (sine, square, triangle, saw, pulse, white noise)
- ADSR envelope control
- WAV file output (24/32-bit, 48kHz stereo)
- S-expression based parameter input
- Command-line parameter interface

## Usage

You can specify parameters either through command-line arguments or using S-expressions.

### Using Command-line Arguments

```bash
cls -w sin -f 440 --dur 2.0 -a 0.1 -d 1.0 -s 0.5 -r 0.05 -o sin.wav
```

### Using S-expressions

```bash
cls --params '((freq . 440) (dur . 2.0) (attack . 0.1) (decay . 1.0) (sustain . 0.5) (release . 0.05) (waveform . "sin"))' -o sin.wav
```

### Options

```
Options:
  --params <PARAMS>                    Input parameters in S-expression format
  -o, --output <OUTPUT>                Output file path [default: output.wav]
  -w, --waveform <WAVEFORM>            Waveform type (sin, square, triangle, saw, pulse, noise)
  -f, --frequency <FREQUENCY>          Frequency in Hz (20-20000)
  --dur <DURATION>                     Duration in seconds
  -a, --attack <ATTACK>                Attack time in seconds
  -d, --decay <DECAY>                  Decay time in seconds
  -s, --sustain <SUSTAIN>              Sustain level (0.0-1.0)
  -r, --release <RELEASE>              Release time in seconds
  --sample-rate <SAMPLE_RATE>          Sample rate in Hz [default: 48000]
  --bits-per-sample <BITS_PER_SAMPLE>  Bits per sample [default: 32]
  -h, --help                           Print help
```

### Integration with Lisp

You can generate parameter S-expressions using Common Lisp:

```lisp
(defun make-tone-params (&key (freq 440)
                             (dur 2.0)
                             (attack 0.1)
                             (decay 1.0)
                             (sustain 0.5)
                             (release 0.05)
                             (waveform "sin"))
  `((freq . ,freq)
    (dur . ,dur)
    (attack . ,attack)
    (decay . ,decay)
    (sustain . ,sustain)
    (release . ,release)
    (waveform . ,waveform)))

(make-tone-params :freq 880 :dur 1.5)
```

## Examples

1. Generate a 440Hz sine wave with default envelope:
```bash
cls -w sin -f 440 --dur 2.0
```

2. Create a noise burst with quick attack and release:
```bash
cls -w noise -a 0.01 -s 0.8 -r 0.1 --dur 1.0
```

3. Generate a square wave with long release:
```bash
cls --params '((freq . 220) (dur . 3.0) (waveform . "square") (release . 2.0))' -o square.wav
```

## License

MIT License

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
