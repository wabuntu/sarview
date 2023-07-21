# sarview
sysstat daily graph

```
$ for file in `ls /var/log/sysstat/* | grep -E "sa[0-9]+$" | sort`; do echo $file; sar -P ALL -C -f $file |  grep Average | tail -n 1; done;


[ -A ] = -bBdFHSvwWy -I SUM -m ALL -n ALL -q ALL -r ALL -u ALL


       -B     Report paging statistics.  The following values are displayed:
              pgpgin/s
              pgpgout/s
              fault/s
              majflt/s
              pgfree/s
              pgscank/s
              pgscand/s
              pgsteal/s
              %vmeff

       -b     Report I/O and transfer rate statistics. The following values are displayed:
              tps    
              rtps   
              wtps   
              dtps   
              bread/s
              bwrtn/s
              bdscd/s


       -d     Report activity for each block device.  
              tps    
              rkB/s  
              wkB/s  
              dkB/s 
              areq-sz
              aqu-sz 
              await  
              %util 
              
        -F [ MOUNT ]
              Display  statistics  for currently mounted filesystems. 
              MBfsfree
              MBfsused
              %fsused
              %ufsused
              Ifree  
              Iused  
              %Iused
              
       -H     Report hugepages utilization statistics.  
              kbhugfree
              kbhugused
              %hugused
              kbhugrsvd
              kbhugsurp

       -I { int_list | SUM | ALL }
              Report statistics for interrupts. 
   
          -m { keyword[,...] | ALL }
              Report power management statistics.
              
       -n { keyword[,...] | ALL }
              Report network statistics.
       
       -P { cpu_list | ALL }
              Report per-processor statistics for the specified processor or processors.  
              
       -q [ keyword[,...] | ALL ]
              Report system load and pressure-stall statistics.                   
              
        -r [ ALL ]
              Report memory utilization statistics.              
                                                      
                     
       -S     Report swap space utilization statistics. 
              kbswpfree
              kbswpused
              %swpused
              kbswpcad
              %swpcad
                     
        -u [ ALL ]
              Report CPU utilization. 
              %user  
              %usr  
              %nice  
              %system
              %sys   
              %iowait
              %steal 
              %irq   
              %soft  
              %guest 
              %gnice 
              %idle                  
                                   
              
       -W     Report swapping statistics. 
              pswpin/s
              pswpout/s
                     
       -w     Report task creation and system switching activity.  
              proc/s
              cswch/s
             
       -y     Report TTY devices activity. The following values are displayed:

              


```
