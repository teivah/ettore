_start:
    text    "".main(SB), ABIInternal, $32-0
    ld      16(g), X10
    bltu    X10, SP, $16
    pcdata  $1, $-1
    pcdata  $0, $-1
    pcdata  $0, $-2
    auipc   $0, $0, X31
    jalr    $0, X31, (X5)(X5)(REG)
    pcdata  $0, $-1
    jal     X0, $-16
    nop
    pcdata  $0, $-2
    sd      X1, -40(SP)
    addi    $-40, SP, SP
    pcdata  $0, $-1
    funcdata        $0, gclocals·33cdeccccebe80329f1fdbee7f5874cb(SB)
    funcdata        $1, gclocals·33cdeccccebe80329f1fdbee7f5874cb(SB)
    addiw   $3, X0, X3
    sd      X3, 8(SP)
    addiw   $2, X0, X5
    sd      X5, 16(SP)
    pcdata  $1, $0
    pcdata  $0, $-2
    auipc   $0, $0, X31
    jalr    $0, X31, (X1)(X1)(REG)
    pcdata  $0, $-1
    ld      24(SP), X3
    sd      X3, ""..autotmp_0+32(SP)
    pcdata  $0, $-2
    auipc   $0, $0, X31
    jalr    $0, X31, (X1)(X1)(REG)
    pcdata  $0, $-1
    ld      ""..autotmp_0+32(SP), X3
    sd      X3, 8(SP)
    pcdata  $0, $-2
    auipc   $0, $0, X31
    jalr    $0, X31, (X1)(X1)(REG)
    auipc   $0, $0, X31
    jalr    $0, X31, (X1)(X1)(REG)
    auipc   $0, $0, X31
    jalr    $0, X31, (X1)(X1)(REG)
    pcdata  $0, $-1
    addiw   $3, X0, X3
    sd      X3, 8(SP)
    sd      X3, 16(SP)
    pcdata  $0, $-2
    auipc   $0, $0, X31
    jalr    $0, X31, (X1)(X1)(REG)
    pcdata  $0, $-1
    ld      24(SP), X3
    sd      X3, ""..autotmp_0+32(SP)
    pcdata  $0, $-2
    auipc   $0, $0, X31
    jalr    $0, X31, (X1)(X1)(REG)
    pcdata  $0, $-1
    ld      ""..autotmp_0+32(SP), X3
    sd      X3, 8(SP)
    pcdata  $0, $-2
    auipc   $0, $0, X31
    jalr    $0, X31, (X1)(X1)(REG)
    auipc   $0, $0, X31
    jalr    $0, X31, (X1)(X1)(REG)
    auipc   $0, $0, X31
    jalr    $0, X31, (X1)(X1)(REG)
    pcdata  $0, $-1
    ld      (SP), X1
    addi    $40, SP, SP
    jalr    $0, X1, X0
    text    "".sum(SB), LEAF|NOFRAME|ABIInternal, $0-24
    funcdata        $0, gclocals·33cdeccccebe80329f1fdbee7f5874cb(SB)
    funcdata        $1, gclocals·33cdeccccebe80329f1fdbee7f5874cb(SB)
    sd      X0, "".~r2+24(SP)
    ld      "".b+16(SP), X3
    ld      "".a+8(SP), X5
    add     X3, X5, X3
    sd      X3, "".~r2+24(SP)
    jalr    $0, X1, X0