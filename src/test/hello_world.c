/* Test that bitwise compound assignment expressions yield the correct value,
 * have the same precedence, and are right-associative.
 */
int main(void) {
    int a = 0;
    if (a > 0)
        if (a > 10)
            return a;
        else
            return 10 - a;
}