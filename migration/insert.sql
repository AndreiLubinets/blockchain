insert into blocks(id, 'from', 'to', value, hash)
values (1, 'test_from', 'test_to', 'test_value', 
  crypto_sha256(crypto_sha256('1test_fromtest_totest_value'))
);

insert into blocks(id, 'from', 'to', value, hash)
values (2, 'test_from2', 'test_to2', 'test_value2', 
  crypto_sha256(crypto_sha256('2test_from2test_to2test_value2'))
);

insert into blocks(id, 'from', 'to', value, hash)
values (3, 'test_from3', 'test_to3', 'test_value3', 
  crypto_sha256(crypto_sha256('3test_from3test_to3test_value3'))
);
