#include <efi.h>
#include "utils.h"

EFI_SYSTEM_TABLE *ST;

/**
 * @brief Prints a string to ConOut.
 *
 * @param str The string to be printed.
 */
VOID Print(CHAR16 *str)
{
    ST->ConOut->OutputString(ST->ConOut, str);
}

EFI_STATUS efi_main(EFI_HANDLE ImageHandle, EFI_SYSTEM_TABLE *SystemTable)
{
    ST = SystemTable;

    EFI_STATUS status;

    EFI_GUID gopGuid = EFI_GRAPHICS_OUTPUT_PROTOCOL_GUID;
    EFI_GRAPHICS_OUTPUT_PROTOCOL *gop;

    status = SystemTable->BootServices->LocateProtocol(&gopGuid, NULL, (void**)&gop);
    if (EFI_ERROR(status))
        Print(L"Unable to locate GOP\r\n");
    else
        Print(L"Located GOP\r\n");

    EFI_GRAPHICS_OUTPUT_MODE_INFORMATION *info;
    UINTN SizeOfInfo, numModes, nativeMode;
    status = gop->QueryMode(gop, gop->Mode == NULL ? 0 : gop->Mode->Mode, &SizeOfInfo, &info);
    if (status == EFI_NOT_STARTED)
        status = gop->SetMode(gop, 0);
    if (EFI_ERROR(status))
    {
        Print(L"Unable to get native mode");
    }
    else
    {
        nativeMode = gop->Mode->Mode;
        numModes = gop->Mode->MaxMode;
    }

    // Print each mode
    for (int i = 0; i < numModes; i++)
    {
        status = gop->QueryMode(gop, i, &SizeOfInfo, &info);
        Print(L"Mode ");
        CHAR16 *Str;
        IntToString(Str, i);
        Print(Str);
        Print(L", width ");
        IntToString(Str, info->HorizontalResolution);
        Print(Str);
        Print(L", height ");
        IntToString(Str, info->VerticalResolution);
        Print(Str);
        Print(L"\r\n");
    }

    while(1);
}
