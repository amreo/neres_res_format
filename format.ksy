meta:
  id: res
  endian: le
seq:
  - id: producter
    contents: "HMRES"
  - id: version 
    type: u1
  - id: magic1
    contents: [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]
  - id: resources_number
    type: u4
  - id: offset_table
    type: resource
    repeat: expr
    repeat-expr: resources_number
types:
  resource:
    seq: 
      - id: offset
        type: u4
    instances:
      content:
        pos: _parent.resources_number*4+offset+20
        size: 10