# roflcat

A faster lolcat. (Unaffiliated with the original lolcat, at
https://github.com/busyloop/lolcat.)

![rainbow catnip](https://i.imgur.com/VHG1CO6m.jpg)

Source: [/r/photoshopbattles](https://www.reddit.com/r/photoshopbattles/comments/ah18qn/psbattle_this_rainbow_cat/eeathte/)

## Why?

I wanted moar rainbowz.

## Benchmarks

Extremely unscientific.

### Measurements
```sh
$ timeout 60 cat /dev/urandom | hexdump -C | target/release/roflcat | pv > /dev/null
3.10GiB 0:01:00 [54.1MiB/s]

$ timeout 60 cat /dev/urandom | hexdump -C | lolcat | pv > /dev/null
1.18GiB 0:01:00 [20.4MiB/s]
```

### Specs

Measured on an AMD Ryzen R5-1600 @ 3.2GHz (12-core) CPU,
on 64-bit Arch Linux.

roflcat and lolcat are single-threaded applications.

```sh
$ uname -a
Linux triangulum 4.19.80-1-lts #1 SMP Fri Oct 18 05:03:40 UTC 2019 x86_64 GNU/Linux

$ rustc --version
rustc 1.38.0 (625451e37 2019-09-23)

$ roflcat --version
roflcat 0.1.0

$ ruby --version
ruby 2.5.7p206 (2019-10-01 revision 67816) [x86_64-linux]

$ lolcat --version
lolcat 100.0.0 (c)2011 moe@busyloop.net
```

## License

Lolcat is licensed under a BSD 3-Clause License. See LICENSES for the full
text and copyright.
