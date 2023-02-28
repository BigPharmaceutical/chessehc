FileOutputStream stream = new FileOutputStream("./font.bin", false)

charErr = null

for (short i = 0; i < 128; i++) {
	stream.write(loadChar(i.toString().padLeft(3, '0') + ".txt"))
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
