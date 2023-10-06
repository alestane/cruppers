#include <iostream>

extern "C" std::wostream* at_cout = &std::wcout;
extern "C" std::wostream* at_cerr = &std::wcerr;

extern "C" bool push_wostream(std::wostream* channel, wchar_t* str)
{
    return static_cast<bool>(*channel << str);
}