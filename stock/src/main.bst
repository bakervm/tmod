(func $main
  (push u8 255)
  (push u8 0)
  (while (> u8)
    (push u8 1)
    (add u8)
    (dup u8)
    (dup u8)
    (u8_promote)
    (dup u16)
    (u16_demote)
    (store u8))
  (sys :write_line)
  (push u8 0)
  (push u8 0)
  (while (== u8)
    (sys :flushfb)))
