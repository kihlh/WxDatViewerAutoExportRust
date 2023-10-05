#define _CRT_SECURE_NO_WARNINGS
#include <string>
#include <windows.h>
#include <codecvt>
#include <regex>
#include <vector>
#include <algorithm>
#include <iterator>
#include <cstddef> // For byte (C++17 or later)

using namespace std;
#define MALLOC(variable) HeapAlloc(GetProcessHeap(), 0, (variable))
#define FREE(variable) HeapFree(GetProcessHeap(), 0, (variable))
#define HMC_CHECK_CATCH catch (char *err){};

#define HMC_VirtualAlloc(Type, leng) (Type) VirtualAlloc((LPVOID)NULL, (DWORD)(leng), MEM_COMMIT, PAGE_READWRITE);
#define HMC_VirtualFree(Virtua) \
    if (Virtua != NULL)         \
        VirtualFree(Virtua, 0, MEM_RELEASE);

// 文本工具
namespace hmc_text_util
{
    string W2A(const wstring &pwText);
    string A2U8(const string &pText);
    string UTF8ToGBK(string u8str);
    bool haslongStr(string Value);
    string W2U8(wstring pwText);
    wstring A2W(const string &paText);
    wstring U82W(const string &pszText);
    string U82A(const string &pText);
    bool haslonglongStr(string Value);
    bool hasIntStr(string Value);
    string base64_encode(const string &input);
    string A2B64A(const string &paText);
    string W2B64A(const wstring &paText);
    wstring W2B64W(const wstring &paText);
    wstring A2B64W(const string &paText);
    const char* A2U8P(const string& pText);
    const char* W2U8P(wstring pwText);


    /**
     * @brief 将A转为bs64
     *
     * @param input
     * @return string
     */
    string base64_encode(const string &input)
    {
        static const string base64_chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

        string encoded;
        try
        {
            int i = 0;
            int j = 0;
            unsigned char array3[3];
            unsigned char array4[4];

            for (char c : input)
            {
                array3[i++] = c;
                if (i == 3)
                {
                    array4[0] = (array3[0] & 0xfc) >> 2;
                    array4[1] = ((array3[0] & 0x03) << 4) + ((array3[1] & 0xf0) >> 4);
                    array4[2] = ((array3[1] & 0x0f) << 2) + ((array3[2] & 0xc0) >> 6);
                    array4[3] = array3[2] & 0x3f;

                    for (int k = 0; k < 4; k++)
                        encoded += base64_chars[array4[k]];

                    i = 0;
                }
            }

            if (i != 0)
            {
                for (int k = i; k < 3; k++)
                    array3[k] = '\0';

                array4[0] = (array3[0] & 0xfc) >> 2;
                array4[1] = ((array3[0] & 0x03) << 4) + ((array3[1] & 0xf0) >> 4);
                array4[2] = ((array3[1] & 0x0f) << 2) + ((array3[2] & 0xc0) >> 6);
                array4[3] = array3[2] & 0x3f;

                for (int k = 0; k < i + 1; k++)
                    encoded += base64_chars[array4[k]];

                while (i++ < 3)
                    encoded += '=';
            }
        }
        catch (char *_)
        {
        }

        return encoded;
    }

    //  WIDE to ANSI
    string W2A(const wstring &pwText)
    {
        string strResult = string();
        try
        {
            if (pwText.empty())
                return strResult;

            int pszATextLen = WideCharToMultiByte(CP_ACP, 0, pwText.c_str(), -1, NULL, 0, NULL, NULL);
            char *pAChar = new (nothrow) char[pszATextLen];
            if (pAChar == NULL)
            {
                return strResult;
            }

            ZeroMemory(pAChar, pszATextLen + 1);
            WideCharToMultiByte(CP_ACP, 0, pwText.c_str(), -1, pAChar, pszATextLen, NULL, NULL);

            strResult.append(pAChar);
            // FreeEnvironmentStringsA(pAChar);
        }
        catch (char *_)
        {
        }

        return strResult;
    }

    /**
     * @brief A字符转为base64字符A
     *
     * @param paText
     * @return string
     */
    string A2B64A(const string &paText)
    {
        string result = string();
        try
        {
            result.append(base64_encode(A2U8(paText)));
        }
        catch (char *_)
        {
        }

        return result;
    }

    /**
     * @brief W字符转为base64字符A
     *
     * @param paText
     * @return string
     */
    string W2B64A(const wstring &paText)
    {
        string result = string();
        try
        {
            result.append(base64_encode(W2U8(paText)));
        }
        catch (char *_)
        {
        }

        return result;
    }

    /**
     * @brief A字符转为base64字符W
     *
     * @param paText
     * @return string
     */
    wstring A2B64W(const string &paText)
    {
        wstring result = wstring();
        try
        {
            result.append(A2W(base64_encode(A2U8(paText))));
        }
        catch (char *_)
        {
        }

        return result;
    }

    /**
     * @brief W字符转为base64字符W
     *
     * @param paText
     * @return string
     */
    wstring W2B64W(const wstring &paText)
    {
        wstring result = wstring();
        try
        {
            result.append(A2W(base64_encode(W2U8(paText))));
        }
        catch (char *_)
        {
        }

        return result;
    }

    //  ANSI to WIDE
    wstring A2W(const string &paText)
    {
        wstring strResult = wstring();
        try
        {

            if (paText.empty())
                return strResult;

            int pszWTextLen = MultiByteToWideChar(CP_ACP, 0, paText.c_str(), -1, NULL, 0);
            wchar_t *pWideChar = new (nothrow) wchar_t[pszWTextLen];

            if (pWideChar == NULL)
                return strResult;

            ZeroMemory(pWideChar, pszWTextLen + 1);
            MultiByteToWideChar(CP_ACP, 0, paText.c_str(), -1, pWideChar, pszWTextLen);

            strResult.append(pWideChar);
            // delete[] pWideChar;
            // pWideChar = NULL;
        }
        catch (char *_)
        {
        }

        return strResult;
    }

    // 宽字符字符串转UTF-8字符串
    string W2U8(wstring pwText)
    {
        string strResult = string();
        try
        {
            if (pwText.empty())
                return strResult;

            int pszATextLen = WideCharToMultiByte(CP_UTF8, 0, pwText.c_str(), -1, NULL, 0, NULL, NULL);
            char *pUTF8 = new char[pszATextLen + 1];
            if (pUTF8 == NULL)
                return strResult;
            ZeroMemory(pUTF8, pszATextLen + 1);
            WideCharToMultiByte(CP_UTF8, 0, pwText.c_str(), -1, pUTF8, pszATextLen, NULL, NULL);
            strResult.append(pUTF8);

            // delete[] pUTF8;
            // pUTF8 = NULL;
        }
        catch (char *_)
        {
        }
        return strResult;
    }

   const char* W2U8P(wstring pwText)
    {
        string result = W2U8(pwText);

        char* pUTF8 = new char[result.size() + 1];

        for (size_t i = 0; i < result.size(); i++)
        {
            char data = result[i];

            if (data == *"\0") {
                pUTF8[i] = data;
                return pUTF8;
            }

            pUTF8[i] = data;
            if (i > result.size()) {
                char end_char = *"\0";
                pUTF8[result.size()] = end_char;
            }
        }

        return pUTF8;
    }
    
    // UTF-8字符串转宽字符
    wstring U82W(const string &pszText)
    {
        wstring strResult = wstring();
        try
        {
            if (pszText.size() == 0)
                return strResult;

            int pszWTextLen = MultiByteToWideChar(CP_UTF8, 0, pszText.c_str(), -1, NULL, NULL);
            wchar_t *pszWText = new wchar_t[pszWTextLen + 1];

            if (pszWText == NULL)
                return strResult;
            ZeroMemory(pszWText, pszWTextLen + 1);
            MultiByteToWideChar(CP_UTF8, 0, pszText.c_str(), -1, pszWText, pszWTextLen);
            strResult.append(pszWText);
            // delete[] pszWText;
            // pszWText = NULL;
        }
        catch (char *_)
        {
        }

        return strResult;
    }

    // 多字节字符串转UTF-8字符串
    string A2U8(const string &pText)
    {
        return W2U8(A2W(pText));
    }

    const char* A2U8P(const string& pText)
    {
        string result = A2U8(pText);

        char* pUTF8 = new char[result.size() + 1];

        for (size_t i = 0; i < result.size(); i++)
        {
            char data = result[i];

            if (data == *"\0") {
                pUTF8[i] = data;
                return pUTF8;
            }

            pUTF8[i] = data;
            if (i > result.size()) {
                char end_char = *"\0";
                pUTF8[result.size()] = end_char;
            }
        }
     
        return pUTF8;
    }

    // UTF-8字符串转多字节字符串
    string U82A(const string &pText)
    {
        return W2A(U82W(pText));
    }

    /**
     * @brief UTF-8 to Base64 encoding ANSI
     *
     * @param pText
     * @return string
     */
    string U82B64A(const string &pText)
    {
        return base64_encode(pText);
    }

    /**
     * @brief UTF-8 to Base64 encoding WIDE
     *
     * @param pText
     * @return string
     */
    wstring U82B64W(const string &pText)
    {
        return A2W(base64_encode(pText));
    }

    // UFT8 字符转为GBK(中文)
    string UTF8ToGBK(string u8str)
    {
        string Result;
        try
        {

            TCHAR *pTempTstr;
            WCHAR *pTempwstr;

            int strSizeTempVar = MultiByteToWideChar(CP_UTF8, 0, u8str.c_str(), -1, NULL, 0);
            pTempwstr = new WCHAR[strSizeTempVar + 1];

            MultiByteToWideChar(CP_UTF8, 0, u8str.c_str(), -1, pTempwstr, strSizeTempVar);
            strSizeTempVar = WideCharToMultiByte(CP_ACP, 0, pTempwstr, -1, NULL, 0, NULL, NULL);

            pTempTstr = new TCHAR[strSizeTempVar + 1];

            WideCharToMultiByte(CP_ACP, 0, pTempwstr, -1, (LPSTR)pTempTstr, strSizeTempVar, NULL, NULL);
            Result = (char *)pTempTstr;
            // delete[] pTempTstr;
            // delete[] pTempwstr;
        }
        catch (char *_)
        {
        }
        return Result;
    }

    // 文本中是否有数字 并且是否是安全的 int32
    bool hasIntStr(string Value)
    {
        bool Result = false;

        if (Value.empty())
            return Result;
        try
        {
            int n = stoi(Value);
            Result = true;
        }
        catch (const exception &e)
        {
            return Result;
        }

        return Result;
    }

    // 文本中是否有数字 并且是否是安全的 long
    bool haslongStr(string Value)
    {
        bool Result = false;
        if (Value.empty())
            return Result;
        try
        {
            long n = stol(Value);
            Result = true;
        }
        catch (const exception &e)
        {
            return Result;
        }

        return Result;
    }

    // 文本中是否有数字 并且是否是安全的 long long
    bool haslonglongStr(string Value)
    {
        bool Result = false;
        if (Value.empty())
            return Result;
        try
        {
            long long n = stoll(Value);
            Result = true;
        }
        catch (const exception &e)
        {
            return Result;
        }

        return Result;
    }

#ifdef defined(_MFC_VER)

    CString UTF8ToCString(string utf8str)
    {
        // 计算所需空间的大小
        int nLen = MultiByteToWideChar(CP_UTF8, NULL,
                                       utf8str.data(), utf8str.size(), NULL, 0);

        // 转换为Unicode
        wstring wbuffer;
        wbuffer.resize(nLen);
        MultiByteToWideChar(CP_UTF8, NULL, utf8str.data(), utf8str.size(),
                            (LPWSTR)(wbuffer.data()), wbuffer.length());

#ifdef UNICODE
        // 如果是Unicode编码，直接返回Unicode字符串
        return (CString(wbuffer.data(), wbuffer.length()));
#else
        /*
         * 转换为ANSI编码
         * 得到转换后长度
         */
        nLen = WideCharToMultiByte(CP_ACP, 0,
                                   wbuffer.data(), wbuffer.length(), NULL, 0, NULL, NULL);

        string ansistr;
        ansistr.resize(nLen);

        // 把Unicode字符串转成ANSI编码字符串
        WideCharToMultiByte(CP_ACP, 0, (LPWSTR)(wbuffer.data()), wbuffer.length(),
                            (LPSTR)(ansistr.data()), ansistr.size(), NULL, NULL);
        return (CString(ansistr.data(), ansistr.length()));
#endif
    }

    string CStringToUTF8(CString strValue)
    {
        wstring wbuffer;
#ifdef _UNICODE
        // 如果是Unicode编码，直接获取Unicode字符串
        wbuffer.assign(strValue.GetString(), strValue.GetLength());
#else
        /*
         * 转换ANSI编码到Unicode编码
         * 获取转换后长度
         */
        int length = MultiByteToWideChar(CP_ACP, MB_ERR_INVALID_CHARS, (LPCTSTR)strValue, -1, NULL, 0);
        wbuffer.resize(length);
        /* 转换 */
        MultiByteToWideChar(CP_ACP, 0, (LPCTSTR)strValue, -1, (LPWSTR)(wbuffer.data()), wbuffer.length());
#endif

        /* 获取转换后长度 */
        int utf8Length = WideCharToMultiByte(CP_UTF8, 0, wbuffer.data(), wbuffer.size(), NULL, 0, NULL, NULL);
        /* 获取转换后内容 */
        string utf8Buffer;
        utf8Buffer.resize(utf8Length);

        WideCharToMultiByte(CP_UTF8, 0, wbuffer.data(), -1, (LPSTR)(utf8Buffer.data()), utf8Length, NULL, NULL);
        return (utf8Buffer);
    }
#endif

}
