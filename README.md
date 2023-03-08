# icommit
Commit with messages generated by AI

# Install

## Using cargo

```
cargo install icommit
```

## Build from source

```
git clone https://github.com/hyf0/icommit.git
cd icommit
cargo install --path .
```

# Usages

Use `ICOMMIT_TOKEN=xxxx` before running `icommit` provide Open API token.

You could also provide the token by create a config file, but I haven't decided yet. 

## Without hints

Just type
```
icommit
```

## With hints

```
icommit "Some hints"
```


# Misc

Currently, icommit is running in verbose mode and will print all logs.