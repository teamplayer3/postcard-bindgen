cargo +nightly run -p gen-bindings --target (rustc -vV | Where-Object { $_ -match 'host: ' } | ForEach-Object { $_ -replace 'host: ' })
