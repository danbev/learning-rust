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
