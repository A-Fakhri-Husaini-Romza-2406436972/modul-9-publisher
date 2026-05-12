## Jawaban

### a. Berapa banyak data yang dikirim publisher dalam satu kali run?
Program publisher mengirim 5 pesan event `user_created` dalam satu kali run. Setiap pesan berisi `user_id` dan `user_name`.

### b. Apa arti URL `amqp://guest:guest@localhost:5672`?
- `amqp` adalah protokol yang digunakan.
- `guest` pertama adalah username.
- `guest` kedua adalah password.
- `localhost:5672` adalah alamat host dan port broker AMQP (biasanya RabbitMQ), dengan `5672` sebagai port default.

![alt text](image.png)

![alt text](image-2.png)

### Penjelasan Screenshot
Berdasarkan screenshot di atas:
- **Terminal kiri (Publisher)** menjalankan perintah `cargo run` dan mengirimkan 5 pesan bertipe `UserCreatedEventMessage` secara berurutan, dengan `user_id` 1 sampai 5 ke broker RabbitMQ.
- **Terminal kanan (Subscriber)** sedang berjalan menunggu pesan (`cargo run`). Ketika publisher mengirim pesan, subscriber berhasil menerima pesan-pesan tersebut dan menampilkannya ("Message received: UserCreatedEventMessage..."). Terdapat teks `In Fakhri's Computer [2406436972]` yang dicetak sebelum pesan diterima, menunjukkan identitas atau tag spesifik dari proses subscriber tersebut.

![alt text](image-3.png)

### Korelasi Spike pada Chart dengan Publisher
Berdasarkan screenshot RabbitMQ Management UI di atas, terdapat lonjakan (spike) pada grafik "Message rates". Spike ini terjadi ketika program `publisher` dijalankan (`cargo run`). Saat program dijalankan, publisher mengirimkan sejumlah pesan (5 event) secara bersamaan ke dalam RabbitMQ. Lonjakan ini mencerminkan aktivitas publish tersebut, di mana grafik menunjukkan peningkatan *publish rate* (pesan per detik) pada saat pesan-pesan tersebut dikirimkan, lalu kembali nol setelah proses selesai.