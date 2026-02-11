# Benchmark

benchmark between rnet and other python http clients


Machine
------

```log
                     ..'          MacBook
                 ,xNMM.           ----------------
               .OMMMMo            OS: macOS Sequoia 15.7.1 arm64
               lMM"               Host: MacBook Pro (16-inch, Nov 2023, Three Thunderbolt 4)
     .;loddo:.  .olloddol;.       Kernel: Darwin 24.6.0
   cKMMMMMMMMMMNWMMMMMMMMMM0:     Uptime: 300 days, 18 hours, 5 mins
 .KMMMMMMMMMMMMMMMMMMMMMMMWd.     Packages: 117 (brew), 11 (brew-cask)
 XMMMMMMMMMMMMMMMMMMMMMMMX.       Shell: zsh 5.9
;MMMMMMMMMMMMMMMMMMMMMMMM:        Display (Color LCD): 3456x2234 @ 120 Hz (as 1728x1117) in]
:MMMMMMMMMMMMMMMMMMMMMMMM:        DE: Aqua
.MMMMMMMMMMMMMMMMMMMMMMMMX.       WM: Quartz Compositor
 kMMMMMMMMMMMMMMMMMMMMMMMMWd.     WM Theme: Blue (Dark)
 'XMMMMMMMMMMMMMMMMMMMMMMMMMMk    Font: .AppleSystemUIFont [System], Helvetica [User]
  'XMMMMMMMMMMMMMMMMMMMMMMMMK.    Cursor: Fill - Black, Outline - White (32px)
    kMMMMMMMMMMMMMMMMMMMMMMd      Terminal: iTerm 3.6.4
     ;KMMMMMMMWXXWMMMMMMMk.       Terminal Font: MesloLGS-NF-Regular (14pt)
       "cooc*"    "*coo'"         CPU: Apple M3 Max (16) @ 4.06 GHz
                                  GPU: Apple M3 Max (40) @ 1.38 GHz [Integrated]
                                  Memory: 41.12 GiB / 128.00 GiB (32%)
                                  Swap: Disabled
                                  Local IP (en0): 192.168.1.172/24
                                  Battery: 75% [AC connected]
                                  Power Adapter: 140W USB-C Power Adapter
                                  Locale: en_US.UTF-8
```

Sync clients
------

- curl_cffi
- requests
- niquests
- pycurl
- [python-tls-client](https://github.com/FlorianREGAZ/Python-Tls-Client.git)
- httpx
- rnet
- ry

Async clients
------

- curl_cffi
- httpx
- niquests
- aiohttp
- rnet
- ry

Target
------


All the clients run with session/client enabled.

## Run benchmark

```bash
# Install dependencies  
pip install -r requirements.txt

# Start server
python server.py

# Start benchmark
python benchmark.py
```
