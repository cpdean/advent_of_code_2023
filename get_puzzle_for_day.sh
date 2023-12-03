#!/usr/bin/env fish
#

set DAY "$argv[1]"


echo $DAY is the day
#curl https://adventofcode.com/2023/day/$1
#
echo "https://adventofcode.com/2023/day/$DAY" 

curl "https://adventofcode.com/2023/day/$DAY" --compressed -H 'User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:120.0) Gecko/20100101 Firefox/120.0' -H 'Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8' -H 'Accept-Language: en-US,en;q=0.5' -H 'Accept-Encoding: gzip, deflate, br' -H 'Referer: https://adventofcode.com/' -H 'Connection: keep-alive' -H 'Cookie: session=53616c7465645f5fb889b69ff941a247c2080e8e1dd4868ea5ba87697ffaed225b053a585e679563d00350cc2b7db11a58bff0ea1279706717bfb4eb8a847d42' -H 'Upgrade-Insecure-Requests: 1' -H 'Sec-Fetch-Dest: document' -H 'Sec-Fetch-Mode: navigate' -H 'Sec-Fetch-Site: same-origin' -H 'Sec-Fetch-User: ?1' -H 'Pragma: no-cache' -H 'Cache-Control: no-cache' -H 'TE: trailers' > tmp.html

set TITLE "(cat tmp.html | htmlq '.day-desc h2')"

printf "## %s\n" "$TITLE" > day$DAY.md

for line in (cat tmp.html | htmlq '.day-desc ' -p -t )
    echo $line
    echo
end | fmt > day$DAY.md



curl "https://adventofcode.com/2023/day/$DAY/input" --compressed -H 'User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:120.0) Gecko/20100101 Firefox/120.0' -H 'Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8' -H 'Accept-Language: en-US,en;q=0.5' -H 'Accept-Encoding: gzip, deflate, br' -H 'Referer: https://adventofcode.com/' -H 'Connection: keep-alive' -H 'Cookie: session=53616c7465645f5fb889b69ff941a247c2080e8e1dd4868ea5ba87697ffaed225b053a585e679563d00350cc2b7db11a58bff0ea1279706717bfb4eb8a847d42' -H 'Upgrade-Insecure-Requests: 1' -H 'Sec-Fetch-Dest: document' -H 'Sec-Fetch-Mode: navigate' -H 'Sec-Fetch-Site: same-origin' -H 'Sec-Fetch-User: ?1' -H 'Pragma: no-cache' -H 'Cache-Control: no-cache' -H 'TE: trailers' > data/$DAY.1.txt
