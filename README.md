[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](https://opensource.org/licenses/MIT)
[![Cargo Build & Test](https://github.com/Antidote1911/argon2cli/actions/workflows/ci.yml/badge.svg)](https://github.com/Antidote1911/argon2cli/actions/workflows/ci.yml)

# argon2cli
A command-line utility to test specific Argon2 instances on your system.

To show usage instructions, run:
```bash
./argon2cli -h
```

### With arguments
```bash
./argon2cli -p password -s somesalt --megabytes 32 --parallel 4 --passes 2 --length 32 --ty argon2i

Password   : password
Salt       : somesalt (in Base64 : c29tZXNhbHQ)
B64 Hash   : z4lrzRLwbTvz3PeAPtgiAhT+8t19TD0tDdVTdHca2o8
Hex Hash   : cf896bcd12f06d3bf3dcf7803ed8220214fef2dd7d4c3d2d0dd55374771ada8f
Hash length : 32
PHC String : $argon2i$v=19$m=32768,t=2,p=4$c29tZXNhbHQ$z4lrzRLwbTvz3PeAPtgiAhT+8t19TD0tDdVTdHca2o8
```

### No arguments or missing some of them
With no arguments or missing some of them, it use this defaults values for speed test:
- p = password
- s = somesalt
- megabytes = 64
- parallel = 4
- passes = 2
- length = 24
- ty = argon2i

```bash
  ./argon2cli

  Password   : password
  Salt       : somesalt (in Base64 : c29tZXNhbHQ)
  B64 Hash   : RdescudvJCsgt3ub+b+dWRWJTmaaJObG
  Hex Hash   : 45d7ac72e76f242b20b77b9bf9bf9d5915894e669a24e6c6
  Hash length : 24
  PHC String : $argon2i$v=19$m=65536,t=2,p=4$c29tZXNhbHQ$RdescudvJCsgt3ub+b+dWRWJTmaaJObG
```

### Thanks to JetBrains for open source support

<a href="https://www.jetbrains.com/"><img src="./jetbrains.png" alt="jetbrains" width="150"></a>


