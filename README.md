# ralp
Convert Apache access_log(combined format) to csv.

## Usage
TODO

## Input
access_log([combined format](https://httpd.apache.org/docs/2.4/ja/logs.html))
```
192.168.0.1 - - [27/Dec/2021:13:48:47 +0900] "GET /foo/index.html HTTP/2.0" 200 1245 "https://example.com" "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/96.0.4664.110 Safari/537.36"                                              |
192.168.0.1 - - [27/Dec/2021:13:48:47 +0900] "GET /bar HTTP/2.0" 200 89521 "https://example.com" "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/96.0.4664.110 Safari/537.36"                                                                  |
...
```

## Output
out.csv
```
"time(utc)","time(local)","remote_host","http_method","http_status","bytes","request_url","referer","user_agent","http_version","remote log name","remote user"
"2021-12-27 04:48:47","2021-12-27 13:48:47 +09:00","192.168.0.1","GET","200","1245","/foo/index.html","https://example.com","Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/96.0.4664.110 Safari/537.36","HTTP/2.0","-","-"
"2021-12-27 04:48:47","2021-12-27 13:48:47 +09:00","192.168.0.1","GET","200","89521","/bar","https://example.com","Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/96.0.4664.110 Safari/537.36","HTTP/2.0","-","-"
...
```
