# fpaq0-rs

fpaq0-rs is a rust version of the fpaq0 arithmetic encoder written by Matt Mahoney.<br>
http://mattmahoney.net/dc/#fpaq0<br>
<br>
To compress:<br>
fpaq0.exe c \input.txt \output.bin<br>
To decompress:<br>
fpaq0.exe d \input.bin \output.txt<br>

# fpaq0f-rs
fpaq0f-rs is a rust version of the fpaq0f adaptive arithmetic coder written by Matt Mahoney. fpaq0f uses a state map in addition to an order-0 context.<br>
<br>
To compress:<br>
fpaq0f.exe c \input.txt \output.bin<br>
To decompress:<br>
fpaq0f.exe d \input.bin \output.txt<br>
