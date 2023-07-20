# sarview
sysstat daily graph

```
$ for file in `ls /var/log/sysstat/* | grep -E "sa[0-9]+$" | sort`; do echo $file; sar -P ALL -C -f $file | grep Average | grep all; done;



- u CPU usage
- P num|ALL for each core
- r memory
- S swap
- b disk IO
- n DEV|EVEV net I/O, EDEV:error

```
