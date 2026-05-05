#!/usr/bin/env bash
# Fails if any fn body in crates/functions/src/features/*/service.rs exceeds 10 lines.
set -euo pipefail
fail=0
for f in $(find crates/functions/src/features -name 'service.rs'); do
    awk '
        /^[[:space:]]*(pub )?(async )?fn [A-Za-z_]/ { in_fn=1; brace=0; body=0; name=$0; next }
        in_fn {
            for (i=1; i<=length($0); i++) {
                c = substr($0, i, 1);
                if (c == "{") brace++;
                if (c == "}") brace--;
            }
            if (brace > 0) {
                trimmed=$0; sub(/^[[:space:]]+/, "", trimmed); sub(/[[:space:]]+$/, "", trimmed);
                if (trimmed != "" && trimmed != "{" && trimmed != "}" && trimmed != "})" && trimmed != "});") body++;
            }
            if (in_fn && brace == 0) {
                if (body > 10) printf("%s:%d  %s\n", FILENAME, NR, name);
                in_fn=0;
            }
        }
    ' "$f"
done | tee /tmp/fn-size-violations.txt
if [ -s /tmp/fn-size-violations.txt ]; then
    echo "FAIL: functions exceeding 10 body lines listed above" >&2
    exit 1
fi
echo "OK: every fn body ≤ 10 lines"
