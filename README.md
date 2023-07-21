# sarview
sysstat daily graph

```
$ for file in `ls /var/log/sysstat/* | grep -E "sa[0-9]+$" | sort`; do echo $file; sar -P ALL -C -f $file |  grep Average | tail -n 1; done;


- u CPU usage
- P num|ALL for each core
- r memory
- S swap
- b disk IO
- n DEV|EVEV net I/O, EDEV:error



[ -A ] = -bBdFHSvwWy -I SUM -m ALL -n ALL -q ALL -r ALL -u ALL
[ -B ] paging
[ -b ] I/O
[ -C ]
[ -d ] each block device
[ -F [ MOUNT ] ] mounted filesystem
[ -H ] hugepage
[ -h ] = pretty, human
[ -r [ ALL ] ] memory
[ -S ] swap
[ -u [ ALL ] ] CPU utilization
[ -v ] inode
[ -W ] swapping
[ -w ] task
[ -y ] tty
[ -I { <int_list> | SUM | ALL } ] interrupts
[ -P { <cpu_list> | ALL } ] cpu, each core
[ -m { <keyword> [,...] | ALL } ] power management
[ -n { <keyword> [,...] | ALL } ] network
[ -q [ <keyword> [,...] | ALL ] ] system load

```
