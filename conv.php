<?php

$a = $argv[1] ?? false;
if ($a === false){
    die("put arg");
}

$opt = $argv[2] ?? false;
if ($opt === false){
    die("put opt");
}
if($opt === '--bin'){
    show_bin($a);
}
elseif($opt === '--hex'){
    show_hex($a);
}
function show_bin($a){
    $a = decbin($a);
    $x = 0;
    for ($i = 0; $i < strlen($a); $i++) {
        $x++;
        echo $a[$i];
        if($x === 4){
            echo ' ';
        }elseif($x === 8){
            echo PHP_EOL;
            $x = 0;
        }
    }
}
function show_hex($a){
    $a = dechex($a);
    $x = 0;
    for ($i = 0; $i < strlen($a); $i++) {
        $x++;
        echo $a[$i];
        if($x === 2){
//             echo ' ';
//         }elseif($x === 4){
            echo PHP_EOL;
            $x = 0;
        }
    }
}