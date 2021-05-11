# OSCH
## **OSC**_ in the \_**S**\_**H**ell
### A bytecode controlled software synthetyzer

### The synth

osch is a monophonic synthethizer. 

It aims to be small and lightweight and be used as a fun way to make alerts when working in the shell.

It uses `cpal` as its audio backend, and atomic loads and stores as a mean of communicating between the audio thread and the "controller thread" that runs a **sequencer**.

## The sequencer

Sequences are encoded as a vector of `Atoms`, the bytecode's instructions.

The language is structured like a tree with simpler instructions as leafs, named `Particles`.

It uses polish notation to compose atoms. An example of sequence could be

```js
loop 3 4 // loop of arity 3 repeated 4 times
A4 1     // An A4 (440hz) that lasts a beat
B2 1/16 
loop 2 2 // "sub" loop of arity 2 repeated 2 times
C4 1
_ 1      // silence for 1 beat
```
which could be written inline as `loop 3 4 A4 1 B2 1/16 loop 2 2 C4 1 _ 1 ` and easily used in the command line. 

abreviating the notes as `A,B,C,_` this would give the sequence

```fix
ABC_C_ABC_C_ABC_C_ABC_C_
```

Other operators could be implemented in the future, like `shuffle`, `reverse`...

...Concepts like relative notes and loops with increments, maybe an `arpeggiate` atom with two arity arguments could allow some way to specify more complex behaviour than a simple loop.

### Compilation

(**TO BE SPECIFIED**) Should the language be actually compiled into bytecode, or parsed from a file before playing ?   Intepreted ? 

We could reckognize being in between valid code and parse as we go, allowing being able to hear the start of our work (the last top level tree node) which could be nice. with our model the main thread would be sleeping anyways

### Inspirations

this is inspired by the spell system in the game [Noita](https://noita.fandom.com/wiki/Guide_To_Wand_Mechanics) by nolla studios

