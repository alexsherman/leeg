# Banana

## Setup

Rocket requires nightly version of cargo, so do 
```
rustup override set nightly
```

Then you should just be able to do ```cargo run``` and be good to go

## Running

If banana has successfully compiled and started running, you can now send requests either using any http client (curl in command line, pythons request library, etc.)

Navigate your browser to localhost:8000
Here, you can test out the basic reqs functionality by requesting a url of the following form:

```
http://localhost:8000/req?team={Comma separated string of champs}&opp={Comma separated string of champs}&req_num={usize}
e.g.
http://localhost:8000/req?team=Annie,Hecarim&opp=Ryze,Leona&req_num=3
```
and see the json response rendered in browser:
```
	{
    0: Kaisa,
    1: Skarner,
    2: Galio
   }
```

