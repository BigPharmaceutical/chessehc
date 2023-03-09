FileWriter writer = new FileWriter("./fontblob.c", false)
charErr = null

writer.write("const unsigned long FONTBLOB_LENGTH=${128 * 24};const unsigned char FONTBLOB[]={")

for (short i = 0; i < 128; i++) {
	byte[] charBytes = loadChar(i.toString().padLeft(3, '0') + ".txt") as byte[]
	for (short j = 0; j < 24; j++) {
		writer.write("0x${String.format("%02X", charBytes[j])}")
		if (i == 127 as short && j == 23 as short) {
			writer.write("};")
		} else {
			writer.write(',')
		}
	}
	writer.flush()
}


def loadChar(String filename) {
	File target = new File("./raw/" + filename)
	if (target.exists()) {
		byte[] result = new byte[24]
		byte resultIndex = 0

		byte acc = 0
		int index = 0
		target.text.bytes.eachWithIndex{ byte entry, int ignored ->
			if (entry < 48 || entry > 51) {
				return
			}
			acc |= switch (entry) {
				case 48 -> 0
				case 49 -> 1
				case 50 -> 2
				case 51 -> 3
			} << (6 - 2 * (index++ % 4))
			if (index % 4 == 0) {
				result[resultIndex++] = acc
				acc = 0
			}
		}
		result
	} else {
		if (charErr == null) {
			charErr = loadChar("err.txt")
		}
		charErr
	}
}
