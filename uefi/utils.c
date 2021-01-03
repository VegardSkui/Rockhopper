#include "utils.h"

/**
 * @brief Converts an integer to a string.
 *
 * @param buffer The string buffer.
 * @param value The integer value.
 */
VOID IntToString(CHAR16 *buffer, INTN value)
{
    // If there is no value, set the string to "0" and return
    if (!value)
    {
        buffer[0] = '0';
        buffer[1] = 0;
        return;
    }

    CHAR16 *sPtr, *dPtr, a[20]; // "a" reserves space for up to 20 characters, enough space for the largest possible 64bit integer
    sPtr = buffer;
    dPtr = a;

    if (value < 0)
    {
        *(sPtr++) = '-';
        value = -value;
    }

    INTN digit;
    while (value)
    {
        digit = value % 10;
        value /= 10;
        CHAR16 digitChar;
        switch (digit)
        {
            case 0:
            digitChar = '0';
            break;
            case 1:
            digitChar = '1';
            break;
            case 2:
            digitChar = '2';
            break;
            case 3:
            digitChar = '3';
            break;
            case 4:
            digitChar = '4';
            break;
            case 5:
            digitChar = '5';
            break;
            case 6:
            digitChar = '6';
            break;
            case 7:
            digitChar = '7';
            break;
            case 8:
            digitChar = '8';
            break;
            case 9:
            digitChar = '9';
            break;
        }
        *(dPtr++) = digitChar;
    }

    // Reverse and append to the real string
    while (dPtr != a)
        *(sPtr++) = *(--dPtr);

    // Terminate the string
    *sPtr = 0;
}
