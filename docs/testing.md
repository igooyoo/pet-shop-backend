
## Ачааллын тест

API сервер хэр зэрэг ачаалал дааж байгааг шалгах тест бөгөөд Apache Bench (ab) ашиглана хийнэ.

- Дэлгэрэнгүй мэдээллийг [эндээс](https://httpd.apache.org/docs/2.4/programs/ab.html) харна уу.


## Эхлүүлэхэд шаардагдах зүйлс

Ачааллын тест хийхдээ дараах зүйлсийг шалгасан байх шаардлагатай.

1. API Server ассан эсэхийг шалгах 
> Хөгжүүлэлтийн орчинд http://127.0.0.1:8536/api/v1/campus хаяг дээр ассан байна.

2. Apache Bench (ab) суулгасан эсэхийг шалгах, Хэрэв суулгаагүй бол доорх линкээс суулгах зааврыг харна уу.
> Дэлгэрэнгүй мэдээллийг https://httpd.apache.org/docs/2.4/programs/ab.html дээр заасан байна.


## Apache Bench (ab) ашиглах

Apache Bench (ab) нь ачааллын тестийг HTTP сервер дээр хялбар хийх боломжтой хэрэгсэл юм.

Дараах коммандыг ажиллуулж үйлдлийг хийнэ:

```
    ab -n 100 -c 10 <url>
```

### Хэрхэн суулгах вэ?

Apache Bench ийн бас нэг давуу тал нь үйлдлийн системээс хамааран аль хэдийн суулгасан байдаг. 
Жнь macOS үйлдлийн систем дээр аль хэдийн суулгасан байдаг, 
хэрэв та linux систем ашигладаг бол `httpd` package суулгасан эсэхийг шалгаж үзэх хэрэгтэй. 
Шалгах `ab -help` коммандыг терминал дээр ажиллуулна. 

Хэрэв суулгаагүй бол `apache2-utils` package ийг суулгах шаардлагатай. Ubuntu үйлдлийн систем дээр жишээ болгон дараах коммандыг оруулж байна.

```
    apt-get update
    apt-get install apache2-utils
```

### Хэрхэн хэрэглэх вэ?

Apache Bench-ийг хамгийн энгийн `ab <url>` коммандаар ажиллуулж болно. Энэ комманд нь нэг хүсэлтээр тухайн `url` рүү хандана, гэхдээ энэ нь API серверт ачаалал үүсгэж чадахгүй.
Бидний API серверт ачаалал үүсгэх зорилготой учраас дараах комплекс тохиргоог ашиглах шаардлагатай болдог.

Их олон сонголт байдаг бөгөөд хамгийн түгээмэл хэдэн тохиргоог танилцуулъяа:

-n: Number of requests
-c: Number of concurrent requests
-H: Add header
—r: flag to not exit on socket receive errors
-k: Use HTTP KeepAlive feature
-p: File containing data to POST
-T: Content-type header to use for POST/PUT data

Дээрх сонголтуудыг ашигласан дараах жишээг харна уу:

```
    ab -n 100 -c 10 -H "Accept-Encoding: gzip, deflate" -rk http://127.0.0.1:8536
```
> :warning: URL дээрх замуудыг `/` тусгаахлах шаарлагатай, эсвэл `ab: invalid URL` гэсэн алдаа өгнө.

Хэрэв POST хүсэлтийг шалгах бол дараах коммандыг ашиглана:

```
    ab -n 100 -c 10 -p data.json -T application/json -rk http://127.0.0.1:8536/api/v1/campus
```

> :bulb: ЗӨВЛӨГӨӨ: Хэрэв тохиргооны бүрэн жагсаалтыг харахыг хүсвэл `ab -help` эсвэл `man ab` коммандыг ашиглаарай. Дэлгэрэнгүй мэдээллийг [онлайн гарын авлагаас](https://httpd.apache.org/docs/2.4/programs/ab.html) харж болно.


### Ачааллын тестийн тайлан

`ab -n 100 -c 10 http://127.0.0.1:8536/api/v1/campus` комманд амжилттай дууссаны дараа дараах тайланг харуулна:
> Тайлбар: http://127.0.0.1:8536/api/v1/campus веб сервис рүү 10 секундын турш 100 зэрэг хандалт үүсгэж шалгах.

```
    Server Software:        
    Server Hostname:        127.0.0.1
    Server Port:            8536

    Document Path:          /api/v1/campus
    Document Length:        95 bytes

    Concurrency Level:      100
    Time taken for tests:   10.626 seconds
    Complete requests:      16063
    Failed requests:        15884
    (Connect: 0, Receive: 0, Length: 15884, Exceptions: 0)
    Non-2xx responses:      15884
    Total transferred:      3451397 bytes
    HTML transferred:       1271841 bytes
    Requests per second:    1511.65 [#/sec] (mean)
    Time per request:       66.153 [ms] (mean)
    Time per request:       0.662 [ms] (mean, across all concurrent requests)
    Transfer rate:          317.19 [Kbytes/sec] received

    Connection Times (ms)
                min  mean[+/-sd] median   max
    Connect:        0    0   0.3      0       9
    Processing:     0   24  63.1     12     790
    Waiting:        0   24  63.0     12     790
    Total:          1   24  63.1     12     790

    Percentage of the requests served within a certain time (ms)
    50%     12
    66%     16
    75%     17
    80%     25
    90%     39
    95%     50
    98%     74
    99%    162
    100%    790 (longest request)
```

Ачааллын тестийн тайлан нь 5 хэсэгтэй байна.

1. Server Software, Server Hostname, Server Port нь таны хандсан API серверийн мэдээлэл байна.
2. Document Path, Document Length нь таны хандсан сервер дээрх веб сервисийн мэдээллийг заана.
3. Энэ хэсгээс хэрэгтэй мэдээллийг авах бөгөөд 
    - `Concurrency Level: ` нэг мөчид нэгэн зэрэг хандаж байгаа хандалтын тоог заана. Жишээ дээр 100 байна.
    - `Time taken for tests:` энэ тестийг хийхэд зарцуулсан хугацаа. Жишээ дээр 10.626 секунд зарцуулсан байна.
    - `Complete requests:` амжилттай болсон хүсэлтийн тоо. Жишээ дээр 16063 байна.
    - `Failed requests:` амжилтгүй болсон хүсэлтийн тоо. Жишээ дээр 15884 байна.
    - `Time per request:` нэг хүсэлтийн зарцуулсан хугацаа. Жишээ дээр 66.153 [ms] (mean) болон 0.662 [ms] (mean, across all concurrent requests) байна.
4. Холболтын хугацааг илэрхийлнэ (Connection Times (ms))
    Энэ жишээ дээр хамгийн хурдан хүсэлт 1 миллисекунд ( *Total* мөрийн *min* багана), хамгийн удаан хүсэлт 790 миллисекунд (*Total* мөрийн *max* багана) -д биелсэн байна. 

5. Хариу ирж байгаа хугацааг тархалтаар харуулсан байна (Percentage of the requests served within a certain time (ms))
    Энэ жишээ дээр нийт хүсэлтийн 50 хувь нь 12 миллисекунд дээр, 99 хувь нь 162 миллисекунд дээр биелсэн байна.