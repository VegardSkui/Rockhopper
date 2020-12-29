#include <efi.h>

CHAR16 *HelloStr = L"Hello World!\r\n";

EFI_STATUS efi_main(EFI_HANDLE ImageHandle, EFI_SYSTEM_TABLE *SystemTable)
{
    SystemTable->ConOut->OutputString(SystemTable->ConOut, HelloStr);
    return EFI_SUCCESS;
}
