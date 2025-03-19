/* SPDX-License-Identifier: MPL-2.0 */

.text
.global context_switch
context_switch: # (cur: *mut TaskContext, nxt: *TaskContext)
    # Save cur's register
    st.d    $ra, $a0, 8*11 # return address
    st.d    $sp, $a0, 8*0
    st.d    $fp, $a0, 8*1
    st.d    $s0, $a0, 8*2
    st.d    $s1, $a0, 8*3
    st.d    $s2, $a0, 8*4
    st.d    $s3, $a0, 8*5
    st.d    $s4, $a0, 8*6
    st.d    $s5, $a0, 8*7
    st.d    $s6, $a0, 8*8
    st.d    $s7, $a0, 8*9
    st.d    $s8, $a0, 8*10

    # Restore nxt's registers
    ld.d    $ra, $a1, 8*11 # return address
    ld.d    $sp, $a1, 8*0
    ld.d    $fp, $a1, 8*1
    ld.d    $s0, $a1, 8*2
    ld.d    $s1, $a1, 8*3
    ld.d    $s2, $a1, 8*4
    ld.d    $s3, $a1, 8*5
    ld.d    $s4, $a1, 8*6
    ld.d    $s5, $a1, 8*7
    ld.d    $s6, $a1, 8*8
    ld.d    $s7, $a1, 8*9
    ld.d    $s8, $a1, 8*10
    ret
