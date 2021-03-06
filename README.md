# fpaq0-rs

fpaq0-rs is a rust version of the fpaq0 arithmetic encoder written by Matt Mahoney.<br>
http://mattmahoney.net/dc/#fpaq0<br>
<br>
To compress:<br>
fpaq0-rs.exe c input output<br>
To decompress:<br>
fpaq0-rs.exe d input output<br>

[Benchmarks](https://sheet.zohopublic.com/sheet/published/i5jwtddd8d29b4ef94fce93450ee6ab9178e5)

<hr>

# fpaq0f-rs
fpaq0f-rs is a rust version of the fpaq0f adaptive arithmetic coder written by Matt Mahoney. fpaq0f uses a state map in addition to an order-0 context.<br>
<br>
To compress:<br>
fpaq0f-rs.exe c input output<br>
To decompress:<br>
fpaq0f-rs.exe d input output<br>

[Benchmarks](https://sheet.zohopublic.com/sheet/published/i5jwtddd8d29b4ef94fce93450ee6ab9178e5)

<hr>

# fpaq0p-rs

fpaq0p-rs is a rust version of the fpaq0p arithmetic encoder written by Ilia Muraviev. Instead of keeping a 0 and 1 count for each context and calculating a prediction like fpaq0, fpaq0p directly keeps a table of predictions for each context and updates a given prediction by adjusting by 1/32 of the error.<br>
<br>
To compress:<br>
fpaq0p-rs.exe c input output<br>
To decompress:<br>
fpaq0p-rs.exe d input output<br>

[Benchmarks](https://sheet.zohopublic.com/sheet/published/i5jwtddd8d29b4ef94fce93450ee6ab9178e5)

<hr>

# fpaq0f-apm-rs
fpaq0f-apm-rs is the same as fpaq0f but with the addition of 5 Adaptive Probability Maps, taken from the bbb compressor by Matt Mahoney.<br>
<br>
To compress:<br>
fpaq0f-apm-rs.exe c input output<br>
To decompress:<br>
fpaq0f-apm-rs.exe d input output<br>

[Benchmarks](https://sheet.zohopublic.com/sheet/published/i5jwtddd8d29b4ef94fce93450ee6ab9178e5)
