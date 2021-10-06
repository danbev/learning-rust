## Embedded Rust notes


### Microcontroller Unit (MCU)
Is a very small computer system self contained on an integrated circuit (IC).
They normally don't have any external data storage available so everything has
to be able to be stored in memory (or perhaps sent off somewhere to be stored
elsewhere).

Is what actually runs our code. It looks like a small black box and has a
number of small metal pins around it. The pins are connected to tracers that
are like wires connecting components on the board.

### Analog-to-digital conversion (ADC)
Microcontrollers convert analog values (continuous) that arrive on inputs (pins)
into to digital values (descrete) to be processed.


### ARM (Advanced RISC Machines/Acorn RISC Machines)
ARM holding is the company behind Arm based chips do not manufacture chips
but instread they design parts of the chip and they license these designs to
hardware manufactures that use them in their products (sometimes enhancing
them). In contrast Intel both design chips and manufacture them.

Arm has a number of families of chip designs. For example there is the Cortex-M
family which are mostly used in microcontrollers.
Cortex-M0 is designed for low cost/power usage.
Cortex-M4 is a middle ground more features/performance than M0 but less
expensive then M7.
Cortex-M7 is higher cost but has more features and better performance.
Some additional information about Arm version can be found
[here](https://github.com/danbev/learning-assembly/tree/master/arm#arm-versions).


### Universal Synchronous and Asynchronous Receiver-Transmitter (USART)
Is a serial communication protocol which as the name suggests can be used
sync (using clocks) or async(using start/stop bits).
```
     Device 1               Device 2
    +----------+           +----------+
    |        TX|-----------|RX        |
    |        RX|-----------|TX        |
    |       GND|-----+-----|GND       |
    +----------+     |     +----------+
                     |
                   -----
                    ---
                     -
```
The sender and reicever must agree on a data transfer rate, which is called the
baud rate which is number of bits transferred per second.
Simplex mode is when data is transferred in one direction only.
Half-duplex is when data is transferred in both directions but not at the same
time.
Full-duplex is when data can be transferred simultainously in both directions at
the same time.

The wire protocol:
```
  +-----------+-------------+---------------+-------------+
  |1 start bit|5-9 data bits|0-1 parity bits|1-2 stop bits|
  +-----------+-------------+---------------+-------------+
```

### Controller Area Network (CAN)
TODO:


### Real Time Operating Systems (RTOS)
TODO:

### Serial communication
Is the process of transmitting one bit of data at a time. Thing of this as being
one wire and the bits flow through it.
Only requires one wire which is one reason at it is used often in
microcontroller system design.

Would require only two wires:
```
  +---------+ b0 b1 b2 b3 b4 b5 b6 b7     +---------+
  |      OUT|-----------------------------|IN       |
  |         | p1 p2 p3 p4 p5 p6 p7 p8     |         |
  |      CLK|-----------------------------|CLK      |
  +---------+                             +---------+

p = puls of the clock
```
So this would send one bit per plus of the clock.

### Parallel communication
Is the process of transmitting multiple bits of data at one time. Think of this
as muliple wires connected to the other side and multiple bits can be set and
transmitted at once.
Since it requires more wires it can be more expensive to implement.
Crosstalk is an issue and it is susceptible to clock skew.
```
  +---------+                             +---------+
  |     OUT0|-------------b0--------------|IN0      |
  |     OUT1|-------------b1--------------|IN1      |
  |     OUT2|-------------b2--------------|IN2      |
  |     OUT3|-------------b3--------------|IN3      |
  |     OUT4|-------------b4--------------|IN4      |
  |     OUT5|-------------b5--------------|IN5      |
  |     OUT6|-------------b6--------------|IN6      |
  |     OUT7|-------------b7--------------|IN7      |
            |             p1              |
  |      CLK|-----------------------------|CLK      |
  +---------+                             +---------+

p = puls of the clock
```
So this would send 8 bit per plus of the clock.

### Synchronous transmission
It's called synchronous because the sender and reciever are synchronized. They
both have a clock which use the same rate.

A clock in this case is something that changes between two values, like high/low
or something like that. The sender might send only when the clock is high for
example. This is not much of a problem if both sender and reciever share the
same clock, if they are part of the same circuit for example, but if they are
not it might be difficult as they need someway of sharing the clock.

Fast transmission (when is has been sent).
May have to wait until data can be sent.

Examples are SPI, and I²C.

### Asynchronous transmission
This type of communication is not synced, but instead it uses a start bit and
and stop bit
```
           start bit    stop bit
               ↓ [ data ]       ↓
    +--------+ 0 11101001 1       +--------+
    | Sender | -----------------> |Reciever|
    +--------+                    +--------+
```
The extra processing of the start/stop bit can affect performance making
asynchronous slower. 
Is cheap and easy to implement (no clock sharing).
Can transmit whenever it is ready (does not have to wait for the clock).


### Clock
Is a signal that repeats/oscillates between high and low states:
```      ____   ____  ____
high 1   |   |  |  |  |  |
         |   |  |  |  |  |    
low  0----   ----  ----  ----  ...
```
So it starts out at zero, changes to 1 for the same amount of time and then
repeats like this.

### Current
Is the flow of free electrons

### Concuctors
Take a conductor like a coppar wire which is good at allowing a current to flow
through it. Now, coppar is made up on coppar atoms which like all atoms contain
protons, neutrons, and electrons:
```
Cu
Protons: 29
Neutrons: 35
Electrons: 29
           Shell 1: 2 electrons
           Shell 2: 8 electrons
           Shell 3: 18 electrons
           Shell 4: 1 electrons (free electron)
```
A coppar wire without any current flowing will have electrons randomly moving
from atom to atom but this is not in any perticular direction. If an electron
moves away from its atom that would leave the atom slightly positively charged
and that would attract an electron. How does the electron leave the atom, this
is a little unclear to me. One thing I've read is that it is actually the atom
that overlap the volume of other atoms and exchange the free atom. So there
does not need to be any energy to cause the electron to move. But I've also read
that this could be influence of room-temprature heat energy.

So we have the sea of electrons that are
moving around randomly. Now, if we connect a cell that contains one end with
negative electrons and the otherside with atoms what are positivly charged (
would attract electrons) the electrons will move towards the positiv end.

The conduction band is the band above the valence band. 
```
----------------------- 
                        Conduction band
-----------------------
                        Band gap
-----------------------
                        Valance band 
-----------------------
        ...
----------------------- Electron shell n-1
```
For electrons to be able to move to an outer shell energy needs to be applied.
For conductors the conduction band and the valance band/band gap/conduction band
overlap allowing the free electrons to move in this space. But for insulators
there is a clear band gap and energy is required for this to happen (that is
if there are free electrons in the valance shell), if we are thinking of atoms
where no energy is applied to it.

Bond:
When an atom bonds with another they can share electrons in their valence shell,
this type of bond is called covalent bond.

Take two silicon atoms for example, they would share a pair of electrons:
```
   *     *                * = electron in valance shell
                         ** = paired electrons
*  Si ** Si *

   *      *
```
Many silicon atoms that are bonded like this will allow for each Si atom to be
connected though sharing four electrons:

This is something that can be seen in silicon where each silicon atom is
connected to four other silicon atoms. 

If a conductor (metal) is heated it becomes less conductive. So metals conduct
electricity better at colder temperatures.

### Band theory
```
Energy
 ^
 |                                           +--------------+
 |                                           | Conduction   |
 |                       +--------------+    |              |
 |                       | Conduction   |    +--------------+
 |   +--------------+    |              |
     | Conduction   |    +--------------+
     |--------------|
     |--------------|    +--------------+    +--------------+
     | Valancy      |    | Valancy      |    | Valancy      |
     +--------------+    +--------------+    +--------------+
     Conductors          Semiconductors      Insulators
```
Notice that for conductors the conduction band and the valancy band overlap
which is why the conduct so well. 
For semiconductors there is a gap between these two bands so energy is needed
for electrons to move into the conduction band. This is possible with heat.


### Insulators
High reisistance to the flow of electricity, which means that their valance
shell is full and 


### Semi-conductors
These have properties that are in between conductors and insulators, hence the
name semi-conductors. If a semi-conductor is exposed to head this can improve it
conductivity. But there is another way to improve the conducitivty of a semi-
conductor which is to add impurities to it (called doping).

If we take a look at a silicon cristal:
```
                                          [* *] = covalent bond
 Si  [* *]  Si  [* *]  Si  [* *]

 ⌈*⌉       ⌈*⌉         ⌈*⌉
 ⌊*⌋       ⌊*⌋         ⌊*⌋
       
 Si  [* *]  Si  [* *]  Si  [* *]

 ⌈*⌉       ⌈*⌉         ⌈*⌉
 ⌊*⌋       ⌊*⌋         ⌊*⌋

 Si  [* *]  Si  [* *]  Si  [* *]
```
So we can see that we have these covelent bounds between the atoms and all of
the free electrons in the valance shell are bound. There are not free electrons
of "holes".

If we replace on of these atoms will an atom with only 5 electrons (like fosfor)
in its valance shell what would happen:
```
                                          [* *] = covalent bond
 Si  [* *]  Si  [* *]  Si  [* *]

 ⌈*⌉       ⌈*⌉         ⌈*⌉
 ⌊*⌋       ⌊*⌋         ⌊*⌋
                 *<-------------------- free electron
 Si  [* *]  P  [* *]  Si  [* *]

 ⌈*⌉       ⌈*⌉         ⌈*⌉
 ⌊*⌋       ⌊*⌋         ⌊*⌋

 Si  [* *]  Si  [* *]  Si  [* *]
```
It can still bound with the other 3 atoms around it but also has an additional
electron left over. This free electron can now move in the conduction band. This
makes this silicon cristal more conductive then before. Because we have a free
electron by this process and electrons are negatively charged this is called
an n-type conductor now.
So that is one way we can increase the connectivity of silicon. But how about
replacing an atom with something that only has 3 electrons in its valance shell:

So lets try with alunminum (Ai):
```
                                          [* *] = covalent bond
 Si  [* *]  Si  [* *]  Si  [* *]

 ⌈*⌉       ⌈*⌉         ⌈*⌉
 ⌊*⌋       ⌊*⌋         ⌊*⌋
                 
 Si  [* *]  Ai  x  *  Si  [* *]

 ⌈*⌉       ⌈*⌉         ⌈*⌉
 ⌊*⌋       ⌊*⌋         ⌊*⌋

 Si  [* *]  Si  [* *]  Si  [* *]
```
Notice that we have a missing electron so thre is no covalent bond between the
Ai atom the Ai atom to the right. This is called a p-type semi-conductor. This
also conducts electricity better than pure silicon so the conductivity goes up.
But notice that this is not because there are free electrons but instead if
there are free electrons they would try to fill the holes. So connecting this
to something that has free electrons would allow for those electrons to move. 
Hmm I'm not sure about the above, I've read that the holes move but I think that
is referring to that electrons could move into the hole which would mean that
there is now a hole somewhere else.

Now, lets see what happens if we combine a p-type and an n-type:
```
                                          [* *] = covalent bond
                                              x = hole
                                              * = free electron

 Si  [* *]  Si  [* *]  Si    |  |  Si  [* *]  Si  [* *]  Si  [* *]
                             |  |
 ⌈*⌉       ⌈*⌉         ⌈*⌉   |  |  ⌈*⌉       ⌈*⌉         ⌈*⌉
 ⌊*⌋       ⌊*⌋         ⌊*⌋   |  |  ⌊*⌋       ⌊*⌋         ⌊*⌋

                             |  |                * 
 Si  [* *]  Ai  x  *  Si     |  |  Si  [* *]  P  [* *]   Si  [* *]
                             |  |
 ⌈*⌉       ⌈*⌉         ⌈*⌉   |  |  ⌈*⌉       ⌈*⌉         ⌈*⌉
 ⌊*⌋       ⌊*⌋         ⌊*⌋   |  |  ⌊*⌋       ⌊*⌋         ⌊*⌋
                             |  |
 Si  [* *]  Si  [* *]  Si    |  |   Si  [* *]  Si  [* *]  Si  [* *]
```
The free electron would be attracted to fill the hole 
```
 Si  [* *]  Si  [* *]  Si    |  |  Si  [* *]  Si  [* *]  Si  [* *]
                             |  |
 ⌈*⌉       ⌈*⌉         ⌈*⌉   |  |  ⌈*⌉       ⌈*⌉         ⌈*⌉
 ⌊*⌋       ⌊*⌋         ⌊*⌋   |  |  ⌊*⌋       ⌊*⌋         ⌊*⌋
                +-------------------------------- * 
                ↓            |  |
 Si  [* *]  Ai  x  *  Si     |  |  Si  [* *]  P  [* *]   Si  [* *]
                             |  |
 ⌈*⌉       ⌈*⌉         ⌈*⌉   |  |  ⌈*⌉       ⌈*⌉         ⌈*⌉
 ⌊*⌋       ⌊*⌋         ⌊*⌋   |  |  ⌊*⌋       ⌊*⌋         ⌊*⌋
                             |  |
 Si  [* *]  Si  [* *]  Si    |  |   Si  [* *]  Si  [* *]  Si  [* *]
```
So the right side was negatively charged before the move as it had a free
electron. 

Before any movement has occured we have the following Formal Charge for P
```
     ⌈*⌉
     ⌊*⌋ *
[* *] P [* *]
     ⌈*⌉
     ⌊*⌋

Bounds: 4
FC = valance electrons - (Bonds + dots)
   =                 5 - (4+1) 
   = 0               
```
And for Ai:
```

     ⌈*⌉
     ⌊*⌋ 
[* *]Ai  x *
     ⌈*⌉
     ⌊*⌋
FC = 3 - (3 + O)
FC = 0
```
Now, after the move of the free electron to the hole we have the following
formal charge for P:
```
FC = 5 - (4 + 0)
   = 1
```
This means that it has a positive formal charge.
And for Ai:
```
     ⌈*⌉
     ⌊*⌋ 
[* *]Ai  [* *]
     ⌈*⌉
     ⌊*⌋

FC = 3 - (4 + O)
FC = 3 - 4
FC = -1
```

```
       P                            N
+--------------------+-------------------------+
| o       o     o    | *    *       *        * |
|     o    o    o    |    *     *     *        |
|  o     o         o |            *       *    |
| o    o   o         | *   *  *     *    *     |
+--------------------+-------------------------+
```

The free electrons will drift to fill in the holes:
```
       P                            N
+--------------------+-------------------------+
| o       o     o   <--*    *       *        * |
|     o    o    o   <--   *     *     *        |
|  o     o         o<--           *       *    |
| o    o   o        <--*   *  *     *    *     |
+--------------------+-------------------------+
```

```
       P                            N
+--------------------+-------------------------+
| o       o     o   -|+*    *       *        * |
|     o    o    o   -|+   *     *     *        |
|  o     o         o-|+           *       *    |
| o    o   o        -|+*   *  *     *    *     |
+--------------------+-------------------------+
```
```
       P                            N
+--------------------+-------------------------+
| o       o     o   | |*    *       *        * |
|     o    o    o   | |   *     *     *        |
|  o     o         o| |           *       *    |
| o    o   o        | |*   *  *     *    *     |
+--------------------+-------------------------+
                     ^
                  Electric field accross the PN junction
```
The electric field makes it difficult for new electrons on the N side to cross
the PN junktion now. The free electrons still have a force acting upon them that
wants to move them to the other side, but there is also a force acting in the
opposite direction. When these forces are equal then no electrons will move over
from the right side to the left side.
So applying a current to the N side that overcomes the force of the PN junction
would allow electrons to move through to the P side and beyond if the P-side
is connected to something. So the flow of electrons can only happen in one
direction.

```
       P                            N
+--------------------+-------------------------+
| o       o     o   | |*    *       *        * |
|     o    o    o   | |   *     *     *        |
|  o     o         o| |           *       *    |
| o    o   o        | |*   *  *     *    *     |
+--------------------+-------------------------+
   Anode                   Cathode

----->|---
 <-- e⁻
            +----+----+  
  anode-----| P  | N  |-----cathode
            +----+----+
```

### Light Emitting Diod
Notice that this is basically a Diode that can emit light. 
```
    +--+
    |  |
    +--+
    |  |
    |  |
    |  |
    |  
Anode  Cathode
```
When we connect these the current must flow through the cathode so it should be
connected to the negative output of the batteri.


### Bipolar Junction Transistors.
There are two (bi) junctions (think of the PN junctions in a diode as explained
above):
```
  +-----+-----+-----+
  |  N  |  P  |  N  |
  +-----+-----+------
        ^     ^  
   junction  junction
       1        2
```
The N is for N-Type, and P for P-Type which are the same as in the diode example
above.
```
      +-----+-----+-----+
E-----|  N  |  P  |  N  |-----C
      +-----+-----+------
               |
               |
               B
E = emitter
C = collector
B = Base
```
So we would have free electrons in the left and right boxes, and holes in the
middle box. And notice if we removed the first box we would be left with a
normal diode:
```
            +-----+-----+
            |  P  |  N  |-----C (cathode)
            +-----+------
               |
               |
               B (anode)
```
And the addition of the first N box is basically the reversal of a diode

Notice that as with the diode for current to flow we would have to connect
E and C to the negative end of cell. And P would connect to the positive.

There are two types of transistors, NPN and PNP types.


### Formal Charge
```
FC = Valance electrons - (Bonds + dots)
```

