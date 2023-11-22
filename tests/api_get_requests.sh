#!/bin/bash
set -e

# Коммандыг ажиллуулахад default-оор `http://127.0.0.1:8536` энэ хаягруу хандана,
# Хэрэв өөр хаягруу хандах бол дараах байдлаар хандах боломжтой.
# Жнь: `./api_get_requests.sh "https://hicampi.com"`.
DOMAIN=${1:-"http://127.0.0.1:8536"}

declare -a arr=(
"/api/v1/health"
#"/api/v1/settings"
)

## Check Apache Bench (ab) Installed
if ! [ -x "$(command -v ab)" ]; then
  echo 'Error: (Apache Bench) is not installed. Follow Apache Bench Documents -> https://httpd.apache.org/docs/2.4/programs/ab.html' >&2
  exit 1
fi

## Checking endpoints.
for path in "${arr[@]}"
do
  URL="$DOMAIN$path"
  printf "\n\n\n"
  echo "testing $URL"
  curl --show-error --fail --silent "$URL" >/dev/null
  ### -c = Сoncurrency, -t = Time (Seconds).
  ab -c 1000 -t 10 -r "$URL" > out.abtest
  grep "Server Hostname:" out.abtest
  grep "Document Path:" out.abtest
  grep "Requests per second" out.abtest
  grep "(mean, across all concurrent requests)" out.abtest
  grep "Transfer rate:" out.abtest
  echo "---"
done

#rm *.abtest
