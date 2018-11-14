<?php

$product = "Product";

?>
<!DOCTYPE html>
<html lang="en" dir="ltr">
  <head>
    <meta charset="utf-8">
    <title><?= $product ?></title>
  </head>
  <body>
    Hello world!
  </body>
</html>

<?php
$time = (microtime(true) - $_SERVER['REQUEST_TIME_FLOAT']) * 1000;
error_log("Time: $time ms");
?>
