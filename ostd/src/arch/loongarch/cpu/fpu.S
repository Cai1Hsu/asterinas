/* SPDX-License-Identifier: MPL-2.0 */

.text
.global fstate_save
fstate_save: # (dst: *mut FpRegs)
    movcf2gr        $t0, $fcc0
    slli.w          $t0, $t0, 1

    movcf2gr        $t0, $fcc1
    slli.w          $t0, $t0, 1

    movcf2gr        $t0, $fcc2
    slli.w          $t0, $t0, 1

    movcf2gr        $t0, $fcc3
    slli.w          $t0, $t0, 1

    movcf2gr        $t0, $fcc4
    slli.w          $t0, $t0, 1

    movcf2gr        $t0, $fcc5
    slli.w          $t0, $t0, 1

    movcf2gr        $t0, $fcc6
    slli.w          $t0, $t0, 1

    movcf2gr        $t0, $fcc7
    st.b            $t0, $a0, 33*8

    fst.d           $f0, $a0, 0*8
    fst.d           $f1, $a0, 1*8
    fst.d           $f2, $a0, 2*8
    fst.d           $f3, $a0, 3*8
    fst.d           $f4, $a0, 4*8
    fst.d           $f5, $a0, 5*8
    fst.d           $f6, $a0, 6*8
    fst.d           $f7, $a0, 7*8
    fst.d           $f8, $a0, 8*8
    fst.d           $f9, $a0, 9*8
    fst.d           $f10, $a0, 10*8
    fst.d           $f11, $a0, 11*8
    fst.d           $f12, $a0, 12*8
    fst.d           $f13, $a0, 13*8
    fst.d           $f14, $a0, 14*8
    fst.d           $f15, $a0, 15*8
    fst.d           $f16, $a0, 16*8
    fst.d           $f17, $a0, 17*8
    fst.d           $f18, $a0, 18*8
    fst.d           $f19, $a0, 19*8
    fst.d           $f20, $a0, 20*8
    fst.d           $f21, $a0, 21*8
    fst.d           $f22, $a0, 22*8
    fst.d           $f23, $a0, 23*8
    fst.d           $f24, $a0, 24*8
    fst.d           $f25, $a0, 25*8
    fst.d           $f26, $a0, 26*8
    fst.d           $f27, $a0, 27*8
    fst.d           $f28, $a0, 28*8
    fst.d           $f29, $a0, 29*8
    fst.d           $f30, $a0, 30*8
    fst.d           $f31, $a0, 31*8

    movfcsr2gr      $t0,    $fcsr0
    st.d            $t0,    $a0, 32*8

    ret

.global fstate_restore
fstate_restore: # (dst: *mut FpRegs)
    fld.d           $f0, $a0, 0*8
    fld.d           $f1, $a0, 1*8
    fld.d           $f2, $a0, 2*8
    fld.d           $f3, $a0, 3*8
    fld.d           $f4, $a0, 4*8
    fld.d           $f5, $a0, 5*8
    fld.d           $f6, $a0, 6*8
    fld.d           $f7, $a0, 7*8
    fld.d           $f8, $a0, 8*8
    fld.d           $f9, $a0, 9*8
    fld.d           $f10, $a0, 10*8
    fld.d           $f11, $a0, 11*8
    fld.d           $f12, $a0, 12*8
    fld.d           $f13, $a0, 13*8
    fld.d           $f14, $a0, 14*8
    fld.d           $f15, $a0, 15*8
    fld.d           $f16, $a0, 16*8
    fld.d           $f17, $a0, 17*8
    fld.d           $f18, $a0, 18*8
    fld.d           $f19, $a0, 19*8
    fld.d           $f20, $a0, 20*8
    fld.d           $f21, $a0, 21*8
    fld.d           $f22, $a0, 22*8
    fld.d           $f23, $a0, 23*8
    fld.d           $f24, $a0, 24*8
    fld.d           $f25, $a0, 25*8
    fld.d           $f26, $a0, 26*8
    fld.d           $f27, $a0, 27*8
    fld.d           $f28, $a0, 28*8
    fld.d           $f29, $a0, 29*8
    fld.d           $f30, $a0, 30*8
    fld.d           $f31, $a0, 31*8

    ld.b            $t0, $a0, 33*8
    movgr2cf        $fcc0, $t0

    srli.w          $t0, $t0, 1
    movgr2cf        $fcc1, $t0

    srli.w          $t0, $t0, 1
    movgr2cf        $fcc2, $t0

    srli.w          $t0, $t0, 1
    movgr2cf        $fcc3, $t0

    srli.w          $t0, $t0, 1
    movgr2cf        $fcc4, $t0

    srli.w          $t0, $t0, 1
    movgr2cf        $fcc5, $t0

    srli.w          $t0, $t0, 1
    movgr2cf        $fcc6, $t0

    srli.w          $t0, $t0, 1
    movgr2cf        $fcc7, $t0

    ld.d           $t0,    $a0, 32*8
    movgr2fcsr     $fcsr0, $t0

    ret
