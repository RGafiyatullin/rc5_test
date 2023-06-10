#!/usr/bin/awk -f

BEGIN {
    print "const CASES: &[&[(&str, &str, &str)]] = &[&["

    currentSet=-1
    currentVec=1
}

function maybe_flush() {
    if (key != "" && plain != "" && cipher != "") {
        print "// set #" currentSet " vec #" currentVec
        print "(\"" key "\", \"" plain "\",\"" cipher "\"),"
        
        key = ""
        plain = ""
        cipher = ""

        currentVec++
        setSizes[currentSet]++
    }
}

/^Set/ {
    if (setNum != $2) {
        if (setNum != "") {
            print "],&["
            currentVec = 0
        }

        currentSet++
        setNum = $2
        setSizes[currentSet] = 0
    }
}

/^.* key=/ {
    key = substr($0, index($0, "=")+1)
    maybe_flush()
}

/^.* plain=/ {
    plain = substr($0, index($0, "=")+1)
    maybe_flush()
}

/^.* cipher=/ {
    cipher = substr($0, index($0, "=")+1)
    maybe_flush()
}

END {
    print "]];"
    print ""

    for (setId in setSizes) {
        print "// Set #" setId " (" setSizes[setId] " vectors)"
        for (vecId = 0; vecId < setSizes[setId]; vecId++) {
            print " // vec #" vecId
            print "#[test]"
            print "fn nessie_" setId "_" vecId "() {"
            print "  let &(key_hex, plaintext_hex, ciphertext_hex) = &CASES[" setId "][" vecId "];"
            print "  run_case::<RC5_32_12_16>(key_hex, plaintext_hex, ciphertext_hex);"
            print "}"
            print ""
        }
    }
}