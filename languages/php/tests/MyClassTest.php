<?php

use PHPUnit\Framework\TestCase;

require_once 'bootstrap.php';


class MyClass
{
    public function concatenateStrings($str1, $str2)
    {
        return $str1 . $str2;
    }
}

class MyClassTest extends TestCase
{
    public function testConcatenateStrings()
    {
        $myClass = new MyClass();
        $str1 = 'hello';
        $str2 = 'world';
        $expectedResult = 'helloworld';

        $result = $myClass->concatenateStrings($str1, $str2);

        $this->assertEquals($expectedResult, $result);
    }
}
