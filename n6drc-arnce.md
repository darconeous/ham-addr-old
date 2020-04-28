%%%
    title = "Amateur Radio Numeric Callsign Encoding"
    abbrev = "N6DRC ARNCE (ham-addr)"
    category = "std"
    docName = "n6drc-arnce-2"
    ipr = "none"
    keyword = ["Ham Address", "EUI-48", "EUI-64", "Link Local", "Callsign", "Amateur Radio", "Ham Radio"]

    date = 2017-09-25T00:00:00Z

    [pi]
    editing = "no"
    private = "yes"
    compact = "yes"
    subcompact = "yes"
    comments = "yes"

    [[author]]
    initials = "R."
    surname = "Quattlebaum"
    fullname = "Robert S. Quattlebaum (N6DRC)"
    role = "editor"
    organization = "Individual"
      [author.address]
      email = "darco@deepdarc.com"
      [author.address.postal]
      region = "California"
      country = "USA"
%%%

.# Abstract

This document describes a mechanism for efficiently and reversibly
encoding radio callsigns into compact numeric identifiers. It also
defines two addressing mechanisms using this numeric encoding: A new
adaptive link-layer addressing scheme (HAM-64) intended for use with
new link-layer networking protocols like ARNGLL (Up to 12 characters),
and an adaptation of EUI-48 and EUI-64 addresses (Up to 8 and 11
characters, respectively) intended for use with existing link layer
protocols.

While this callsign encoding/addressing schemes was developed for
amateur radio purposes, there is no technical reason why it could not
be used directly with other radio services that make use of ITU
callsigns.

.# Copyright Notice

Copyright (C) 2017 Robert Quattlebaum. All rights reserved.

This use of this document is hereby granted under the terms of the
Creative Commons International Attribution 4.0 Public License, as
published by Creative Commons.

 *  <https://creativecommons.org/licenses/by/4.0/>

This work is provided as-is. Unless otherwise provided in writing, the
authors make no representations or warranties of any kind concerning
this work, express, implied, statutory or otherwise, including without
limitation warranties of title, merchantability, fitness for a
particular purpose, non infringement, or the absence of latent or
other defects, accuracy, or the present or absence of errors, whether
or not discoverable, all to the greatest extent permissible under
applicable law.

.# Implementations of this standard

The above copyright notice and license applies only to this specific
document. The implementation of mechanisms described herein, as well
as the replication and use of any contained datasets required for the
implementation of said mechanisms, are offered freely to be used for
any purpose, public or private, commercial or non-commercial.

{mainmatter}

# Introduction #

Radio callsigns are used to identify the source (and possibly the
destination) of licensed radio transmissions. Radio callsigns are
generally assigned to individuals or organizations for identification
purposes by the local regulatory authority, such as the FCC in the
United States.

Their presence in transmissions is often mandated by regulations,
which works well for voice and constant-wave (CW or morse-code) modes,
but doesn't work well for packet radio. AX.25, for example, imposes a
hard 6-character limit on callsign length. The longest callsign that
can currently be assigned to an individual is seven characters long,
which means that operators which are assigned seven-character
callsigns cannot legally use AX.25. Additionally, the callsign is
encoded as plain ASCII text, which is hardly the most compact
encoding.

Naively encoding a six-character callsign using ASCII requires at
least 48 bits. However if we encoded each character using just six
bits, this would make a six-character callsign take up only 36 bits.
This is certainly an improvement, but we can still do better.

Callsigns exclusively consist of a collection of letters (A thru Z)
and numeric digits (0 thru 9). There are also secondary suffixes which
can be appended with the slash character `/`, giving us a total of 37
individual symbols that can be present in a callsign. We can use this
limited character set to our advantage.

It turns out that if we limit ourselves to fewer than 40 different
possible symbols, we can store up to three characters in a 16-bit
block. Thus, we can store a six character callsign in just 32 bits, a
nine character callsign in 48 bits and a twelve character callsign in
64 bits. 12 characters is plenty of space for both a callsign *and* an
indicator suffix/prefix.

This document takes the basic premise described above and fleshes it
out into a full addressing specification by fully defining the
following:

*   A base-40 character set optimized for encoding ITU radio callsigns
*   A method for encoding three base-40 values (three callsign
    characters) within a 16-bit (two-byte) integer
*   A new 64-bit addressing format ('HAM-64') intended for use with
    with new link-layer packet radio protocols; capable of encoding
    callsigns up to 12 characters long
*   EUI-48 and EUI-64 callsign encodings intended for use with
    existing link-layer protocols; capable of encoding callsigns up to
    8 and 11 characters long


# Base-40 Character Set

Since there are a limited number of valid characters in an ITU radio
callsign, we represent each character in a callsign as number in
base-40. The character set we are using for this document defined below:

No. | Char | No. | Char | No. | Char | No. | Char
:--:|:----:|:---:|:----:|:---:|:----:|:---:|:----:
0   | *NUL*| 10  | `J`  | 20  | `T`  | 30  | `3`
1   | `A`  | 11  | `K`  | 21  | `U`  | 31  | `4`
2   | `B`  | 12  | `L`  | 22  | `V`  | 32  | `5`
3   | `C`  | 13  | `M`  | 23  | `W`  | 33  | `6`
4   | `D`  | 14  | `N`  | 24  | `X`  | 34  | `7`
5   | `E`  | 15  | `O`  | 25  | `Y`  | 35  | `8`
6   | `F`  | 16  | `P`  | 26  | `Z`  | 36  | `9`
7   | `G`  | 17  | `Q`  | 27  | `0`  | 37  | `/`
8   | `H`  | 18  | `R`  | 28  | `1`  | 38  | `-`
9   | `I`  | 19  | `S`  | 29  | `2`  | 39  | *ESC*/`^`

Where *NUL* is analogous to the ASCII *NUL* character, and *ESC* is
reserved for future use with a currently undefined and experimental
character escaping mechanism. (Should be rendered as `^`)

While this character set was designed specifically for use with the
16-bit chunk encoding defined in the next section, it is possible
that this character set could be useful for other purposes.

# 16-Bit Chunk Encoding {#chunk-encoding}

Using the previously defined character set, we can store three characters
as a single integer that fits within a 16-bit integer using the encoding:

    S = CHAR[0] * 1600 + CHAR[1] * 40 + CHAR[2]

Where `CHAR[0]` is the first (leftmost) character and `CHAR[2]` is the
last (rightmost) character in the "chunk".

We can then decode the original characters by reversing that process:

    CHAR[0] = S / 1600 MOD 40
    CHAR[1] = S / 40 MOD 40
    CHAR[2] = S MOD 40

When encoding a chunk that uses less than three characters, you would
set `CHAR[2]` and/or `CHAR[1]` (as appropriate) to the value 0
(*NUL*), indicating that there is not a character present in that
position.

The maximum valid value using this three-character 16-bit encoding is
0xF9FF. Any 16-bit value larger than 0xF9FF **MUST NOT** be decoded
using the above mechanism. The such values **MAY** be used to indicate
other conditions, or simply be treated as invalid. For example, the
HAM-64 address format uses values larger than 0xF9FF in the first
chunk as a way to indicate that the address is a special address.

To encode callsigns longer than six characters, multiple 16-bit chunks
are concatenated.

While this 16-bit base-40 encoding mechanism was designed specifically
for use with the HAM-64 address format defined in the next section, it
is possible that this encoding could be useful for other purposes.

# HAM-64 Address Format

HAM-64 is a new link-layer addressing format, similar in purpose to
the ubiquitous EUI-48 and EUI-64 address standards. It was specifically
designed for compactness and encoding clarity. It's features include:

*   Encodes callsigns up to 12 characters long, allowing for long
    complex callsigns
*   Variable length encoding, allowing for 16-bit, 32-bit, 48-bit, and
    64-bit representations of 3, 6, 9, and 12 character callsigns
*   Broadcast and multicast address support

## Address Construction

HAM-64 addresses are encoded as four 16-bit big-endian "chunks",
which can contain up to three characters in each chunk. Chunks are
always stored in big-endian order.

When writing out a HAM-64 address, each chunk is shown as a four-digit
hexadecimal number, with each chunk separated by a colon `:`.

Lets have a look at a relatively short callsign, `N6DRC`:

 *  `N6D` encodes to `0x5CAC`
 *  `RC ` encodes to `0x70F8`

Thus, the full ham address for this callsign is `5CAC:70F8:0000:0000`.

By convention, all trailing chunks with the value `0x0000` are omitted
for brevity. Because the HAM-64 encoding of `N6DRC` has two trailing
zero chunks, `5CAC:70F8` is the preferred notation.

The process is identical for large callsigns, like `VI2BMARC50`:

 *  `VI2` encodes to `0x8B05`
 *  `BMA` encodes to `0x0E89`
 *  `RC5` encodes to `0x7118`
 *  `0  ` encodes to `0xA8C0`

Thus, the ham address for this callsign is `8B05:0E89:7118:A8C0`.
Since it is so long, there is no shorter representation.

## Adaptive Address Size

Since few amateur radio callsigns are longer than six characters, it
is expected that link-layer protocols using HAM-64 will want to
take the notation convention mentioned above to heart. By including
the source and destination address sizes in the link-layer header,
it is possible to omit all trailing zero-valued chunks from
header. This allows for common callsign lengths to be compactly
encoded without sacrificing the flexibility of using a longer
callsign when necessary.

## Special HAM-64 Addresses

All addresses where the first chunk is greater than or equal to 0xFA00
are *special addresses*. Special addresses do not have a callsign
representations. These special addresses used for multicast and
broadcast for link layers that use ham addresses natively. All values
and ranges which are not explicitly defined below are to be considered
**RESERVED**.

### Empty Callsign

The all-zero address (zero-length callsign) is **RESERVED** and
**MUST NOT** be sent over the wire.

### Broadcast

The broadcast (all-nodes) address for link-layers using HAM-64
addresses is defined as `FFFF:0000:0000:0000`.

### IPv6 Multicast

For link-layers using HAM-64 addresses for link-layer addressing,
IPv6 multicast MUST be implemented using addresses of the format
`FAxx:xxxx:xxxx:xxxx`, where `x` represents a special *reversed*
representation (defined below) of the IPv6 group-id.

For converting IPv6 multicast addresses into link-local HAM-64
multicast addresses, you MUST store the lower 13 octets of the
multicast group-id *in reverse order*. This takes advantage of the
fact that IPv6 multicast addresses tend to be zero-filled, allowing
for compact HAM-64 address representations. For example, a multicast
address of `ff02::1` would simply be the abbreviated ham address
`FA01`.

### IPv4 Multicast ###

IPv4 multicast can be implemented for link-layers using HAM-64
addresses using addresses of the format `FBxx:xxxx:0000:0000`, where
`x` represents the byte values for the last three octets of the IPv4
multicast address.

# EUI-48 and EUI-64 #

Sometimes it is useful to encode a callsign in an EUI-48 or EUI-64
address. This can be useful when operating standard Wi-Fi or 802.15.4
equipment under section 97 rules. While it is mathematically
impossible to encode every HAM-64 address as either an EUI-48 or an
EUI-64, a significant subset of addresses can be encoded: 8 characters
for EUI-48 and 11 characters for EUI-64. Under certain circumstances,
a full 9 or 12 characters can be encoded.

One of the goals of this encoding is to allow fast translation between
a "ham-address" and its associated EUI-64 or EUI-48---specifically, no
multiplication or division is required. Thus, 61 of the total 64-bits
have a direct one-to-one representation in an EUI-64 representation,
with the last three bits assumed to be zero (which they will be if the
last character is *NUL*).

## Encoding Process ##

While there are some other minor caveats (see EUI-64 section below),
the basic process for encoding works like this:

1.  Rotate the address by 8 bits to the right (so that the last byte
    becomes the first byte)
2.  Set the least-significant three bits of the first byte to '0',
    '1', '0'.

**NOTE:** Special ham addresses (defined in a section above) MUST NOT
be encoded as a EUI-48 or EUI-64 using this scheme! EUI-48 and EUI-64
addresses have their own multicast/broadcast mapping, which must be
used instead.

## Decoding Process ##

Not considering the minor caveats regarding decoding EUI-48-derived
EUI-64 addresses (See EUI-64 section below), the basic process for
decoding works like this:

1.  Verify the least significant bits of the first byte are 0, 1, 0.
2.  Verify the second byte is greater than 0x05. (Not strictly necessary,
    but will cause many non-callsign-encoded addresses to fail early)
3.  Clear the least-significant three bits of the first byte.
4.  Rotate the address by 8 bits to the left (so that the first byte
    becomes the last byte)
5.  Verify that each chunk is less than 0xFA00.
6.  Verify that the first chunk is greater than 0x063F.
7.  Verify that, when all characters are decoded, all characters after
    the first *NUL* are also *NUL*.

If any of the verify steps above fail then the address is either
corrupted or simply does not contain an encoded callsign.

Care was taken to ensure that a EUI-48-encoded ham address that has
been converted to a EUI-64 address does not parse correctly unless
converted to a EUI-48 first.

## EUI-48

This encoding allows us to encode an 8-digit callsign in a EUI-48:

       7 --Octet 1-- 0   7 --Octet 2-- 0   7 --Octet 3-- 0
    M +---------------+ +---------------+ +---------------+ ...
    S |C|C|C|C|C|0|1|0| |A|A|A|A|A|A|A|A| |A|A|A|A|A|A|A|A| ...
    B +---------------+ +---------------+ +---------------+ ...

         7 --Octet 4-- 0   7 --Octet 5-- 0   7 --Octet 6-- 0
    ... +---------------+ +---------------+ +---------------+ L
    ... |B|B|B|B|B|B|B|B| |B|B|B|B|B|B|B|B| |C|C|C|C|C|C|C|C| S
    ... +---------------+ +---------------+ +---------------+ B

Where:

 *  `A` is the 16-bit ham address encoding of the first, second, and
    third characters of the callsign. Valid values are between
    `0x0640` and `0xF9FF`, inclusive.
 *  `B` is the 16-bit ham address encoding of the fourth, fifth, and
    sixth characters of the callsign. Valid values are between
    `0x0000` and `0xF9FF`, inclusive.
 *  `C` is the most-significant 13-bits of the 16-bit ham address
    encoding of the seventh and eighth characters of the callsign. The
    least-significant three bits are always assumed to be zero. Note
    that octet 1 contains bits 7 thru 3 and octet 6 contains bits 15
    thru 8.

So, for `N6DRC`, `5CAC:70F8:0000:0000` becomes `02:5C:AC:70:F8:00`.

## EUI-64

The encoding of an EUI-64 address is similar to the encoding of the
EUI-48 address, except that it can hold callsigns up to 11 characters.

If the callsign is 8 characters or less, then you **MUST** encode your
callsign as an EUI-48 and convert it to an EUI-64 using the standard
EUI-48 to EUI-64 mechanism (using `0xFFFE` for the padding value). For
callsigns larger than 8 characters, we follow similar logic that we
did when constructing the EUI-48:

       7 --Octet 1-- 0   7 --Octet 2-- 0   7 --Octet 3-- 0
    M +---------------+ +---------------+ +---------------+ ...
    S |D|D|D|D|D|0|1|0| |A|A|A|A|A|A|A|A| |A|A|A|A|A|A|A|A| ...
    B +---------------+ +---------------+ +---------------+ ...

                7 --Octet 4-- 0   7 --Octet 5-- 0
           ... +---------------+ +---------------+ ...
           ... |B|B|B|B|B|B|B|B| |B|B|B|B|B|B|B|B| ...
           ... +---------------+ +---------------+ ...

         7 --Octet 6-- 0   7 --Octet 7-- 0   7 --Octet 8-- 0
    ... +---------------+ +---------------+ +---------------+ L
    ... |C|C|C|C|C|C|C|C| |C|C|C|C|C|C|C|C| |D|D|D|D|D|D|D|D| S
    ... +---------------+ +---------------+ +---------------+ B

Where:

 *  `A` is the 16-bit ham address encoding of the first, second, and
    third character of the callsign. Valid values are between `0x0640`
    and `0xF9FF`, inclusive.
 *  `B` is the 16-bit ham address encoding of the fourth, fifth, and
    sixth characters of the callsign. Valid values are between
    `0x0000` and `0xF9FF`, inclusive.
 *  `C` is the 16-bit ham address encoding of the seventh, eighth, and
    ninth characters of the callsign. Valid values are between
    `0x0000` and `0xF9FF`, inclusive.
 *  `D` is the most-significant 13-bits of the 16-bit ham address
    encoding of the tenth and eleventh characters of the callsign. The
    least-significant three bits are always assumed to be zero. Note
    that octet 1 contains bits 7 thru 3 and octet 6 contains bits 15
    thru 8.

So, from our original examples:

 *  `N6DRC`: `5CAC:70F8:0000:0000` becomes `02:5C:AC:FF:FE:70:F8:00`
 *  `VI2BMARC50`: `8B05:0E89:7118:A8C0` becomes
    `C2:8B:05:0E:89:71:18:A8`

## 9-Char EUI-48 and 12-Char EUI-64 {#eui-hack}

The astute reader may realize that there are a few 9-character
callsigns which *can* be encoded into an EUI-48. Likewise, there are a
few 12-character callsigns which can be encoded in an EUI-64. The
trick is that the integer representation of the last character must be
evenly divisible by 8.

Thus, if any of the following characters are the last character of a 9
or 12 character callsign, it can still be represented fully as an
EUI-48 or EUI-64: `H`, `P`, `X`, or `5`.

The issue of whether such a 9-character address should be required
to be encoded as a EUI-48 or a EUI-64 is to be decided. In the
interim, assume EUI-64.

{backmatter}

# Examples and Test Vectors

## Callsigns

 *  `N6DRC`:
    *  HAM-64: `5CAC:70F8:0000:0000`
    *  EUI-48: `02:5C:AC:70:F8:00`
    *  EUI-64: `02:5C:AC:FF:FE:70:F8:00`
 *  `KJ6QOH/P`:
    *  HAM-64: `4671:6CA0:F000:0000`
    *  EUI-48: `02:46:71:6C:A0:F0`
    *  EUI-64: `02:46:71:FF:FE:6C:A0:F0`
 *  `D9K`:
    *  HAM-64: `1EAB:0000:0000:0000`
    *  EUI-48: `02:1E:AB:00:00:00`
    *  EUI-64: `02:1E:AB:FF:FE:00:00:00`
 *  `NA1SS`:
    *  HAM-64: `57C4:79B8:0000:0000`
    *  EUI-48: `02:57:C4:79:B8:00`
    *  EUI-64: `02:57:C4:FF:FE:79:B8:00`
 *  `VI2BMARC50`:
    *  HAM-64: `8B05:0E89:7118:A8C0`
    *  EUI-48: N/A
    *  EUI-64: `C2:8B:05:0E:89:71:18:A8`

## IPv6 Multicast

 * `ff02::1`
    *  HAM-64: `FA01:0000:0000:0000`
    *  EUI-48: `33:33:00:00:01`

# References and Links

 * [Amateur Radio Next Generation Link Layer (ARNGLL)](https://gist.github.com/darconeous/e1c7c352098671a36230)
 *  <http://www.ng3k.com/Dxcc/dxcc.html>
 *  <https://tools.ietf.org/html/rfc5234>
