# sarview
sysstat daily graph

```
$ for file in `ls /var/log/sysstat/* | grep -E "sa[0-9]+$" | sort`; do echo $file; sar -P ALL -C -f $file | grep Average | grep all; done;


sar -S -b ALL -C
sar -r ALL -C -f sar20230715
sar -n ALL -C -f sa20230715
sar -b ALL -C -f sa20230715
sar -B ALL -C -f sa20230715
sar -b -C -f sa20230715

```
