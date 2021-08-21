# fpaq0-rs

fpaq0-rs is a rust version of the fpaq0 arithmetic encoder written by Matt Mahoney.<br>
http://mattmahoney.net/dc/#fpaq0<br>
<br>
To compress:<br>
fpaq0-rs.exe c input output<br>
To decompress:<br>
fpaq0-rs.exe d input output<br>

[Benchmarks](https://sheet.zoho.com/sheet/open/1pcxk88776ef2c512445c948bee21dcbbdba5?sheet=Sheet1&range=A1)

<hr>

# fpaq0f-rs
fpaq0f-rs is a rust version of the fpaq0f adaptive arithmetic coder written by Matt Mahoney. fpaq0f uses a state map in addition to an order-0 context.<br>
<br>
To compress:<br>
fpaq0f-rs.exe c input output<br>
To decompress:<br>
fpaq0f-rs.exe d input output<br>

[Benchmarks](https://sheet.zoho.com/sheet/open/1pcxk88776ef2c512445c948bee21dcbbdba5?sheet=Sheet1&range=A1)

# fpaq0p-rs

fpaq0p-rs is a rust version of the fpaq0p arithmetic encoder written by Ilia Muraviev. Instead of keeping a 0 and 1 count for each context and calculating a prediction like fpaq0, fpaq0p directly keeps a table of predictions for each context and updates a given prediction by adjusting by 1/32 of the error.<br>
<br>
To compress:<br>
fpaq0-rs.exe c input output<br>
To decompress:<br>
fpaq0-rs.exe d input output<br>

[Benchmarks](https://sheet.zoho.com/sheet/open/1pcxk88776ef2c512445c948bee21dcbbdba5?sheet=Sheet1&range=A1)
