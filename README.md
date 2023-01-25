## C Extension [PSQL 15]

╭─  ~/_Code/FABMation/PostgreSQL/semver-rust  ✔     master 4 
╰─ pgbench -f bench/range.sql --log --transactions=10000 --username=postgres postgres --host 127.0.0.1                                                                                                                 19:43:37   32.4G 
pgbench (14.6, server 15.1)
starting vacuum...pgbench: error: ERROR:  relation "pgbench_branches" does not exist
pgbench: (ignoring this error and continuing anyway)
pgbench: error: ERROR:  relation "pgbench_tellers" does not exist
pgbench: (ignoring this error and continuing anyway)
pgbench: error: ERROR:  relation "pgbench_history" does not exist
pgbench: (ignoring this error and continuing anyway)
end.
transaction type: bench/range.sql
scaling factor: 1
query mode: simple
number of clients: 1
number of threads: 1
number of transactions per client: 10000
number of transactions actually processed: 10000/10000
latency average = 0.062 ms
initial connection time = 3.938 ms
tps = 16213.289060 (without initial connection time)

╭─  ~/_Code/FABMation/PostgreSQL/semver-rust  ✔     master 5 
╰─ e bench/comp.sql                                                                                                                                                                                                    19:44:20   32.4G 

╭─  ~/_Code/FABMation/PostgreSQL/semver-rust  ✔     master 5 
╰─ pgbench -f bench/comp.sql --log --transactions=10000 --username=postgres postgres --host 127.0.0.1                                                                                                        19:44:26   1.74s   32.3G 
pgbench (14.6, server 15.1)
starting vacuum...pgbench: error: ERROR:  relation "pgbench_branches" does not exist
pgbench: (ignoring this error and continuing anyway)
pgbench: error: ERROR:  relation "pgbench_tellers" does not exist
pgbench: (ignoring this error and continuing anyway)
pgbench: error: ERROR:  relation "pgbench_history" does not exist
pgbench: (ignoring this error and continuing anyway)
end.
transaction type: bench/comp.sql
scaling factor: 1
query mode: simple
number of clients: 1
number of threads: 1
number of transactions per client: 10000
number of transactions actually processed: 10000/10000
latency average = 0.050 ms
initial connection time = 4.435 ms
tps = 19972.916725 (without initial connection time)


## Rust (Release Mode)

### PSQL 14

╰─ pgbench -f bench/range.sql --log --transactions=200000 --username=postgres postgres --host 127.0.0.1                                                                                                     20:39:05   12.74s   33.7G 
pgbench (14.6)
starting vacuum...pgbench: error: ERROR:  relation "pgbench_branches" does not exist
pgbench: (ignoring this error and continuing anyway)
pgbench: error: ERROR:  relation "pgbench_tellers" does not exist
pgbench: (ignoring this error and continuing anyway)
pgbench: error: ERROR:  relation "pgbench_history" does not exist
pgbench: (ignoring this error and continuing anyway)
end.
transaction type: bench/range.sql
scaling factor: 1
query mode: simple
number of clients: 1
number of threads: 1
number of transactions per client: 200000
number of transactions actually processed: 200000/200000
latency average = 0.054 ms
initial connection time = 3.555 ms
tps = 18507.150330 (without initial connection time)



╰─ pgbench -f bench/comp.sql --log --transactions=200000 --username=postgres postgres --host 127.0.0.1                                                                                                                 20:38:26   33.7G 
pgbench (14.6)
starting vacuum...pgbench: error: ERROR:  relation "pgbench_branches" does not exist
pgbench: (ignoring this error and continuing anyway)
pgbench: error: ERROR:  relation "pgbench_tellers" does not exist
pgbench: (ignoring this error and continuing anyway)
pgbench: error: ERROR:  relation "pgbench_history" does not exist
pgbench: (ignoring this error and continuing anyway)
end.
transaction type: bench/comp.sql
scaling factor: 1
query mode: simple
number of clients: 1
number of threads: 1
number of transactions per client: 200000
number of transactions actually processed: 200000/200000
latency average = 0.050 ms
initial connection time = 2.922 ms
tps = 19909.751089 (without initial connection time)
