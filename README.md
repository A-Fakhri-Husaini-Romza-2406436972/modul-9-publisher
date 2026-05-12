## Jawaban

### a. Berapa banyak data yang dikirim publisher dalam satu kali run?
Program publisher mengirim 5 pesan event `user_created` dalam satu kali run. Setiap pesan berisi `user_id` dan `user_name`.

### b. Apa arti URL `amqp://guest:guest@localhost:5672`?
- `amqp` adalah protokol yang digunakan.
- `guest` pertama adalah username.
- `guest` kedua adalah password.
- `localhost:5672` adalah alamat host dan port broker AMQP (biasanya RabbitMQ), dengan `5672` sebagai port default.

![alt text](image.png)