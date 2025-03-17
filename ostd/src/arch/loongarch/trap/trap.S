/* SPDX-License-Identifier: MPL-2.0 */

.section .text
.global trap_entry
.balign 4096
trap_entry:
    csrwr   $sp,    0x30            # SAVE0, save sp(and load the trap context, for user trap)
    bnez    $sp,    save_context

.begin_trap_from_kernel:
    csrrd   $sp,    0x30
    addi.d  $sp,    $sp, -35*8      # allocate stack space for kernel trap frame

save_context:
    # Save general registers except for sp
    st.d    $ra,    $sp, 1*8
    st.d    $tp,    $sp, 2*8

    st.d    $a0,    $sp, 4*8
    st.d    $a1,    $sp, 5*8
    st.d    $a2,    $sp, 6*8
    st.d    $a3,    $sp, 7*8
    st.d    $a4,    $sp, 8*8
    st.d    $a5,    $sp, 9*8
    st.d    $a6,    $sp, 10*8
    st.d    $a7,    $sp, 11*8
    st.d    $t0,    $sp, 12*8
    st.d    $t1,    $sp, 13*8
    st.d    $t2,    $sp, 14*8
    st.d    $t3,    $sp, 15*8
    st.d    $t4,    $sp, 16*8
    st.d    $t5,    $sp, 17*8
    st.d    $t6,    $sp, 18*8
    st.d    $t7,    $sp, 19*8
    st.d    $t8,    $sp, 20*8
    st.d    $r21,   $sp, 21*8
    st.d    $fp,    $sp, 22*8
    st.d    $s0,    $sp, 23*8
    st.d    $s1,    $sp, 24*8
    st.d    $s2,    $sp, 25*8
    st.d    $s3,    $sp, 26*8
    st.d    $s4,    $sp, 27*8
    st.d    $s5,    $sp, 28*8
    st.d    $s6,    $sp, 29*8
    st.d    $s7,    $sp, 30*8
    st.d    $s8,    $sp, 31*8

    # Save prmd, era, badv and t0
    csrrd   $t0,    0x1             # PRMD
    st.d    $t0,    $sp, 32*8

    csrrd   $t0,    0x6             # ERA
    st.d    $t0,    $sp, 33*8

    csrrd   $t0,    0x7             # BADV
    st.d    $t0,    $sp, 34*8

    # save user sp
    csrrd   $t0,    0x30            # SAVE0
    st.d    $t0,    $sp, 3*8

    csrrd   $t0,    0x1             # PRMD
    andi    $t0,    $t0, 0b11       # extract PPLV field
    bnez    $t0,    trap_from_user
    # b       trap_from_kernel

trap_from_kernel:
    move    $a0,    $sp             # trap frame as first argument
    bl      trap_handler
    li.d    $t0,    0x0             # clear SAVE0, indicating that the next trap is from kernel
    csrwr   $t0,    0x30
    b       trap_return

trap_from_user:
    ld.d    $sp,    $sp, 0*8
    ld.d    $fp,    $sp, 1*8
    ld.d    $s0,    $sp, 2*8
    ld.d    $s1,    $sp, 3*8
    ld.d    $s2,    $sp, 4*8
    ld.d    $s3,    $sp, 5*8
    ld.d    $s4,    $sp, 6*8
    ld.d    $s5,    $sp, 7*8
    ld.d    $s6,    $sp, 8*8
    ld.d    $s7,    $sp, 9*8
    ld.d    $s8,    $sp, 10*8
    ld.d    $ra,    $sp, 11*8

    addi.d   $sp,    $sp, 12*8

    ret

.global run_user
run_user: # (ctx: &mut UserContext)
    addi.d  $sp,    $sp, -12*8
    st.d    $sp,    $sp, 0*8    # zero was used to save kernel sp
    st.d    $fp,    $sp, 1*8
    st.d    $s0,    $sp, 2*8
    st.d    $s1,    $sp, 3*8
    st.d    $s2,    $sp, 4*8
    st.d    $s3,    $sp, 5*8
    st.d    $s4,    $sp, 6*8
    st.d    $s5,    $sp, 7*8
    st.d    $s6,    $sp, 8*8
    st.d    $s7,    $sp, 9*8
    st.d    $s8,    $sp, 10*8
    st.d    $ra,    $sp, 11*8

    move    $t0,    $sp
    move    $sp,    $a0
    st.d    $t0,    $sp, 0*8
    csrwr   $sp,    0x30            # SAVE0

trap_return:
    ld.d    $t0,    $sp, 33*8       # ERA
    csrwr   $t0,    0x6
    ld.d    $t0,    $sp, 32*8       # PRMD
    csrwr   $t0,    0x1

    # Restore general registers except for sp
    ld.d    $ra,    $sp, 1*8
    ld.d    $tp,    $sp, 2*8

    ld.d    $a0,    $sp, 4*8
    ld.d    $a1,    $sp, 5*8
    ld.d    $a2,    $sp, 6*8
    ld.d    $a3,    $sp, 7*8
    ld.d    $a4,    $sp, 8*8
    ld.d    $a5,    $sp, 9*8
    ld.d    $a6,    $sp, 10*8
    ld.d    $a7,    $sp, 11*8
    ld.d    $t0,    $sp, 12*8
    ld.d    $t1,    $sp, 13*8
    ld.d    $t2,    $sp, 14*8
    ld.d    $t3,    $sp, 15*8
    ld.d    $t4,    $sp, 16*8
    ld.d    $t5,    $sp, 17*8
    ld.d    $t6,    $sp, 18*8
    ld.d    $t7,    $sp, 19*8
    ld.d    $t8,    $sp, 20*8
    ld.d    $r21,   $sp, 21*8
    ld.d    $fp,    $sp, 22*8
    ld.d    $s0,    $sp, 23*8
    ld.d    $s1,    $sp, 24*8
    ld.d    $s2,    $sp, 25*8
    ld.d    $s3,    $sp, 26*8
    ld.d    $s4,    $sp, 27*8
    ld.d    $s5,    $sp, 28*8
    ld.d    $s6,    $sp, 29*8
    ld.d    $s7,    $sp, 30*8
    ld.d    $s8,    $sp, 31*8

    ld.d    $sp,    $sp, 3*8

    ertn
