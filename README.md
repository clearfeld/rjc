# rjc

```bash
dir | rjc dir
```

```json
{"meta":{"drive":"F","serial":"38EC-3395","directory":"F:\\Dev\\rjc","files":3,"directories":4},"resources":[{"date":"01/15/2023","time":"04:14 PM","is_dir":true,"size":null,"name":"."},{"date":"01/15/2023","time":"04:14 PM","is_dir":true,"size":null,"name":".."},{"date":"01/14/2023","time":"04:25 PM","is_dir":false,"size":8,"name":".gitignore"},{"date":"01/15/2023","time":"10:38 PM","is_dir":false,"size":11117,"name":"Cargo.lock"},{"date":"01/20/2023","time":"12:53 AM","is_dir":false,"size":437,"name":"Cargo.toml"},{"date":"01/21/2023","time":"02:18 PM","is_dir":true,"size":null,"name":"src"},{"date":"01/20/2023","time":"12:53 AM","is_dir":true,"size":null,"name":"target"}]}
```

## Parsers

### Win32

| Commands  | Documentation                                            |
| --------- | -------------------------------------------------------- |
| assoc     | [details](https://rjc.vercel.app/parsers/win32/assoc)    |
| dir       | [details](https://rjc.vercel.app/parsers/win32/dir)      |
| netstat   | [details](https://rjc.vercel.app/parsers/win32/netstat)  |
| tasklist  | [details](https://rjc.vercel.app/parsers/win32/tasklist) |

### Unix

| Commands  | Documentation                                         |
| --------- | ----------------------------------------------------- |
| cksum     | [details](https://rjc.vercel.app/parsers/unix/cksum)  |
| du        | [details](https://rjc.vercel.app/parsers/unix/du)     |
| env       | [details](https://rjc.vercel.app/parsers/unix/env)    |
| file      | [details](https://rjc.vercel.app/parsers/unix/file)   |
| ls        | [details](https://rjc.vercel.app/parsers/unix/ls)     |
| shadow    | [details](https://rjc.vercel.app/parsers/unix/shadow) |
| wc        | [details](https://rjc.vercel.app/parsers/unix/wc)     |

### Darwin

| Commands    | Documentation                                            |
| ----------- | -------------------------------------------------------- |
| airport     | [details](https://rjc.vercel.app/parsers/darwin/airport) |

### External

| Commands                                     | Documentation                                        |
| -------------------------------------------- | ---------------------------------------------------- |
| [lsd](https://github.com/Peltoche/lsd)       | [details](https://rjc.vercel.app/parsers/common/lsd) |

### Formats

| Commands          | Documentation                                            |
| ----------------- | -------------------------------------------------------- |
| email-address     | [details](https://rjc.vercel.app/parsers/formats/email)  |
