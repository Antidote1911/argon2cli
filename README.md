# argon2cli
A command-line utility to test specific Argon2 instances on your system.  
Latest Windows, Linux and other release are [here](https://github.com/Antidote1911/argon2cli/releases/latest).

To show usage instructions, run:
```bash
./argon2cli -h
```

### With arguments
```bash
./argon2cli --pass password --salt somesalt -m 32 -p 4 -i 2 -l 32 -t argon2i

Password   : password
Salt       : somesalt (in Base64 : c29tZXNhbHQ)
Memory: 32 KiB, Iterations: 2, Parallelism: 4, Hash length: 32, Algo: Argon2i)
===================================
Hex hash   : 85724a55cdca41f067be4d9f68dfcc5d0289a5f664fd96b0d6777a65672080ed
PHC String : $argon2i$v=19$m=32,t=2,p=4$c29tZXNhbHQ$hXJKVc3KQfBnvk2faN/MXQKJpfZk/Zaw1nd6ZWcggO0
Execution Time : 0.017565563 s
```

### No arguments or missing some of them
With no arguments or missing some of them, it use this defaults values for speed test:  
- pass = password  
- salt = somesalt
- memory = 65536 KiB
- parallelism = 4
- iterations = 2
- hash length = 24
- argon type = argon2i

```bash
./argon2cli

Password   : password
Salt       : somesalt (in Base64 : c29tZXNhbHQ)
Memory: 65536 KiB, Iterations: 2, Parallelism: 4, Hash length: 24, Algo: Argon2i)
===================================
Hex hash   : 45d7ac72e76f242b20b77b9bf9bf9d5915894e669a24e6c6
PHC String : $argon2i$v=19$m=65536,t=2,p=4$c29tZXNhbHQ$RdescudvJCsgt3ub+b+dWRWJTmaaJObG
Execution Time : 1.6075927989999999 s
```

For use in script, the flags --oh (only output hash) or --op (only output phc string) are very usefull.
```bash
./argon2cli --op
$argon2i$v=19$m=65536,t=2,p=4$c29tZXNhbHQ$RdescudvJCsgt3ub+b+dWRWJTmaaJObG

./argon2cli --oh
45d7ac72e76f242b20b77b9bf9bf9d5915894e669a24e6c6
```
