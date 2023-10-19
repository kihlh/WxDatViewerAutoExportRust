#ifndef HMC_IMPORT_REGISTR_H
#define HMC_IMPORT_REGISTR_H

#include <windows.h>
#include <iostream>
#include <set>
#include <string>
#include <map>
#include <vector>
#include <type_traits>
#include <ShlObj.h>
using namespace std;

#define MAX_KEY_LENGTH 255
#define MAX_VALUE_NAME 16383

#define _define_if_to_break(eq1, result)                      \
    {                                                         \
        if (!is_ok || pDataSize == 0 || is_open == 0 || !eq1) \
            return result;                                    \
    }

#define _define_is_int32bit(T)         \
    (                                  \
        is_same_v<T, int64_t> ||       \
        is_same_v<T, long long> ||     \
        is_same_v<T, int32_t> ||       \
        is_same_v<T, size_t> ||        \
        is_same_v<T, unsigned long> || \
        is_same_v<T, HWND> ||          \
        is_same_v<T, long> ||          \
        is_same_v<T, long int> ||      \
        is_same_v<T, unsigned long>)

#define _define_is_int64bit(T)         \
    (                                  \
        is_same_v<T, int64_t> ||       \
        is_same_v<T, long long> ||     \
        is_same_v<T, int32_t> ||       \
        is_same_v<T, size_t> ||        \
        is_same_v<T, unsigned long> || \
        is_same_v<T, HWND> ||          \
        is_same_v<T, long> ||          \
        is_same_v<T, long int> ||      \
        is_same_v<T, unsigned long>)

// 关闭注册表键
#define _defined_auto_free_HKey(subHKey)            \
    shared_ptr<void> close_key(nullptr, [&](void *) \
                               {\
        if (subHKey != nullptr) {\
            ::RegCloseKey(subHKey);\
            subHKey = nullptr;\
        } });

namespace hmc_registr
{
    // 目录的信息
    struct chQueryDirStat
    {
        string path; // 路径
        string hkey; // 根名称
        bool success;
        long long LastWriteTime; // 上次写入时间的时间戳
    };

    // 枚举键
    struct chQueryDirKey
    {
        vector<string> key;
        vector<string> dir;
    };

    // 遍历树结构的信息 但是不返回内容
    struct chWalkItme
    {
        DWORD size;         // 值的大小
        string vkey;        // 值的名称
        string dirPath;     // 路径文件夹
        DWORD type;         // 类型
        HKEY root;          // 根路径
        bool isDir;         // 是否是文件夹
        long long time;     // 时间戳
        vector<BYTE> value; // 数据
        bool is_value;      // 是否加入了数据
    };

    // 获取值的信息
    struct chValueStat
    {
        DWORD type;
        DWORD size;
        bool exists;
    };

    void _lib_EnumRegistrKeyQuery(HKEY hKey, vector<string> &QueryDirList, vector<string> &QueryKeyList);
    HKEY getHive(string hkey);
    string getHive(HKEY hkey);
    chQueryDirStat getRegistrDirStat(HKEY hKey, string path);
    chQueryDirKey listKey(HKEY hKey, string path);
    bool path2hKey(string path, HKEY &hKey, string &p_path);

    /**
     * @brief 对比两个注册表类型能否被隐式转换
     *
     * @param reType 实际类型
     * @param targetType 强制转换为
     * @return true
     * @return false
     */
    bool _EQ_REG_TYPE(DWORD reType, DWORD targetType)
    {
        switch (reType)
        {
        // 文本
        case REG_LINK:
        case REG_SZ:
        case REG_MULTI_SZ:
        case REG_EXPAND_SZ:
        {
            return targetType == REG_NONE || targetType == REG_LINK || targetType == REG_SZ || targetType == REG_MULTI_SZ || targetType == REG_EXPAND_SZ;
        }
        case REG_DWORD_BIG_ENDIAN:
        case REG_QWORD:
        case REG_DWORD:
        {
            return targetType == REG_NONE || targetType == REG_DWORD_BIG_ENDIAN || targetType == REG_QWORD || targetType == REG_DWORD;
        }
        case REG_BINARY:
        case REG_RESOURCE_LIST:
        case REG_RESOURCE_REQUIREMENTS_LIST:
        {
            return targetType == REG_NONE || targetType == REG_BINARY || targetType == REG_RESOURCE_LIST || targetType == REG_RESOURCE_REQUIREMENTS_LIST;
        }

        default:
            return true;
        }
    }

    // ----------------------------------------------------------------------------------------------
    /**
     * @brief 枚举注册表的key
     *
     * @param hKey
     * @param QueryDirList
     * @param QueryKeyList
     */
    void _lib_EnumRegistrKeyQuery(HKEY hKey, vector<string> &QueryDirList, vector<string> &QueryKeyList)
    {
        try
        {
            char achKey[MAX_KEY_LENGTH];    // 子键名称的缓冲区
            DWORD cbName = 0;               // 名称字符串的大小
            char achClass[MAX_PATH] = "";   // 类名缓冲区
            DWORD cchClassName = MAX_PATH;  // 类字符串的大小
            DWORD cSubKeys = 0;             // 子键数
            DWORD cbMaxSubKey = 0;          // 最长子键大小
            DWORD cchMaxClass = 0;          // 最长类字符串
            DWORD cValues = 0;              // 键值的个数
            DWORD cchMaxValue = 0;          // 最长值名
            DWORD cbMaxValueData = 0;       // 最长值数据
            DWORD cbSecurityDescriptor = 0; // 安全描述符的大小
            FILETIME ftLastWriteTime;       // 最后写入时间

            char achValue[MAX_VALUE_NAME];   // key存储
            DWORD cchValue = MAX_VALUE_NAME; // 数据序号

            DWORD index, retCode;

            // 获取类名和值计数。

            retCode = RegQueryInfoKeyA(
                hKey,                  // key句柄
                achClass,              // 类名缓冲区
                &cchClassName,         // 类字符串的大小
                NULL,                  // 无
                &cSubKeys,             // 子键数
                &cbMaxSubKey,          // 最长子键大小
                &cchMaxClass,          // 最长类字符串
                &cValues,              // 键值的个数
                &cchMaxValue,          // 最长值名
                &cbMaxValueData,       // 最长值数据
                &cbSecurityDescriptor, // 安全描述符的大小
                &ftLastWriteTime);     // 最后写入时间

            // 枚举子键，直到RegEnumKeyEx失败。
            if (cSubKeys)
            {

                for (index = 0; index < cSubKeys; index++)
                {
                    cbName = MAX_KEY_LENGTH;
                    retCode = RegEnumKeyExA(hKey, index,
                                            achKey,
                                            &cbName,
                                            NULL,
                                            NULL,
                                            NULL,
                                            &ftLastWriteTime);
                    if (retCode == ERROR_SUCCESS)
                    {
                        QueryDirList.push_back(achKey);
                    }
                }
            }
            // 枚举键值。
            if (cValues)
            {
                for (index = 0, retCode = ERROR_SUCCESS; index < cValues; index++)
                {
                    cchValue = MAX_VALUE_NAME;
                    achValue[0] = '\0';
                    retCode = RegEnumValueA(hKey, index,
                                            achValue,
                                            &cchValue,
                                            NULL,
                                            NULL,
                                            NULL,
                                            NULL);

                    if (retCode == ERROR_SUCCESS)
                    {
                        string str = string(achKey);
                        str.resize(cchValue);
                        for (size_t i = 0; i < cchValue; i++)
                            str[i] = achValue[i];

                        QueryKeyList.push_back(str);
                    }
                }
            }
        }
        catch (const exception &e)
        {
        }
    }
    // ----------------------------------------------------------------------------------------------

    /**
     * @brief 反HKEY解析 因为要和napi兼容
     *
     * @param hkey
     * @return HKEY
     */
    HKEY getHive(string hkey)
    {
        if (hkey == "HKEY_CURRENT_USER")
        {
            return HKEY_CURRENT_USER;
        }

        if (hkey == "HKEY_LOCAL_MACHINE")
        {
            return HKEY_LOCAL_MACHINE;
        }

        if (hkey == "HKEY_CLASSES_ROOT")
        {
            return HKEY_CLASSES_ROOT;
        }

        if (hkey == "HKEY_USERS")
        {
            return HKEY_USERS;
        }

        if (hkey == "HKEY_CURRENT_CONFIG")
        {
            return HKEY_CURRENT_CONFIG;
        }

        return NULL;
    }

    /**
     * @brief 反HKEY解析 因为要和napi兼容
     *
     * @param hkey
     * @return string
     */
    string getHive(HKEY hkey)
    {
        if (hkey == HKEY_CURRENT_USER)
        {
            return "HKEY_CURRENT_USER";
        }

        if (hkey == HKEY_LOCAL_MACHINE)
        {
            return "HKEY_LOCAL_MACHINE";
        }

        if (hkey == HKEY_CLASSES_ROOT)
        {
            return "HKEY_CLASSES_ROOT";
        }

        if (hkey == HKEY_USERS)
        {
            return "HKEY_USERS";
        }

        if (hkey == HKEY_CURRENT_CONFIG)
        {
            return "HKEY_CURRENT_CONFIG";
        }

        return "";
    }

    /**
     * @brief 获取目录的信息多少个 key 更新时间
     *
     * @param hKey
     * @param path
     * @return chQueryDirStat
     */
    chQueryDirStat getRegistrDirStat(HKEY hKey, string path)
    {
        chQueryDirStat queryDirStat = {
            getHive(hKey),
            path, false, 0};

        try
        {
            HKEY hTestKey;
            _defined_auto_free_HKey(hTestKey);
            if (RegOpenKeyExA(hKey, path.c_str(),
                              0,
                              KEY_ALL_ACCESS,
                              &hTestKey) == ERROR_SUCCESS)
            {
                char achKey[MAX_KEY_LENGTH];    // 子键名称的缓冲区
                DWORD cbName = 0;               // 名称字符串的大小
                char achClass[MAX_PATH] = "";   // 类名缓冲区
                DWORD cchClassName = MAX_PATH;  // 类字符串的大小
                DWORD cSubKeys = 0;             // 子键数
                DWORD cbMaxSubKey = 0;          // 最长子键大小
                DWORD cchMaxClass = 0;          // 最长类字符串
                DWORD cValues = 0;              // 键值的个数
                DWORD cchMaxValue = 0;          // 最长值名
                DWORD cbMaxValueData = 0;       // 最长值数据
                DWORD cbSecurityDescriptor = 0; // 安全描述符的大小
                FILETIME ftLastWriteTime;       // 最后写入时间

                RegQueryInfoKeyA(
                    hKey,                  // key句柄
                    achClass,              // 类名缓冲区
                    &cchClassName,         // 类字符串的大小
                    NULL,                  // 无
                    &cSubKeys,             // 子键数
                    &cbMaxSubKey,          // 最长子键大小
                    &cchMaxClass,          // 最长类字符串
                    &cValues,              // 键值的个数
                    &cchMaxValue,          // 最长值名
                    &cbMaxValueData,       // 最长值数据
                    &cbSecurityDescriptor, // 安全描述符的大小
                    &ftLastWriteTime);     // 最后写入时间

                const ULONGLONG epochOffset = 116444736000000000ULL;
                ULARGE_INTEGER uli;
                uli.LowPart = ftLastWriteTime.dwLowDateTime;
                uli.HighPart = ftLastWriteTime.dwHighDateTime;
                ULONGLONG timestamp = (uli.QuadPart - epochOffset) / 10000ULL;
                queryDirStat.success = true;
                queryDirStat.LastWriteTime = static_cast<time_t>(timestamp);
            }
        }
        catch (const exception &e)
        {
        }
        return queryDirStat;
    }

    /**
     * @brief 获取目录的信息多少个 key 更新时间
     *
     * @param path
     * @return chQueryDirStat
     */
    chQueryDirStat getRegistrDirStat(string path)
    {
        HKEY hKey;
        string p_path;
        path2hKey(path, hKey, p_path);
        return getRegistrDirStat(hKey, path);
    }

    /**
     * @brief 枚举key
     *
     * @param hKey 根HKEY
     * @param string 路径
     * @return vector<string>
     */
    chQueryDirKey listKey(HKEY hKey, string path)
    {
        chQueryDirKey queryDirKey;

        try
        {
            HKEY hTestKey;
            _defined_auto_free_HKey(hTestKey);
            if (RegOpenKeyExA(hKey, path.c_str(),
                              0,
                              KEY_ALL_ACCESS,
                              &hTestKey) == ERROR_SUCCESS)
            {
                _lib_EnumRegistrKeyQuery(hTestKey, queryDirKey.dir, queryDirKey.key);
            }
        }
        catch (const exception &e)
        {
        }

        return queryDirKey;
    }

    /**
     * @brief 枚举key
     *
     * @param path
     * @return chQueryDirKey
     */
    chQueryDirKey listKey(string path)
    {
        HKEY hKey;
        string p_path;
        path2hKey(path, hKey, p_path);
        return listKey(hKey, path);
    }

    /**
     * @brief 分割文本
     *
     * @param path
     * @return vector <string>
     */
    vector<string> _lib_splitString(string path, string sep = "\\")
    {
        vector<string> result;
        string::size_type startPos = 0;
        string::size_type endPos = path.find(sep);

        while (endPos != string::npos)
        {
            result.push_back(path.substr(startPos, endPos - startPos));
            startPos = endPos + 1;
            endPos = path.find(sep, startPos);
        }

        result.push_back(path.substr(startPos));
        return result;
    }

    /**
     * @brief 路径合并为常规路径
     *
     * @param paths
     * @return string
     */
    string _lib_joinString(vector<string> paths, string sep = "")
    {

        string newStr = string();

        for (size_t i = 0; i < paths.size(); i++)
        {
            string path = paths[i];
            newStr.append(path);
            if (i != paths.size() - 1)
                newStr.append(sep);
        }

        return newStr;
    }

    /**
     * @brief 完整路径分析出HKEY和路径
     *
     * @param path
     * @param hKey
     * @param p_path
     * @return true
     * @return false
     */
    bool path2hKey(string path, HKEY &hKey, string &p_path)
    {
        bool result = false;
        size_t pos = path.find('\\');
        if (pos != 0)
        {
            string key1 = path.substr(0, pos);
            HKEY hive = getHive(key1);
            if (hive != NULL)
            {
                hKey = hive;
            }
            p_path.clear();
            p_path.append(path.substr(pos, path.size() + 1));
            result = true;
        }
        return result;
    }

    /**
     * @brief 获取值类型与值路径
     *
     * @param hKey 根
     * @param path 路径
     * @param valueType 传址 DW
     * - 0x00000000 REG_NONE 未定义
     * - 0x00000001 REG_SZ 字符串
     * - 0x00000002 REG_EXPAND_SZ 未展开引用的字符串 例如“%PATH%”
     * - 0x00000003 REG_BINARY 二进制
     * - 0x00000004 REG_DWORD / REG_DWORD_LITTLE_ENDIAN 32 位数字
     * - 0x00000005 REG_DWORD_BIG_ENDIAN 大端格式的 32 位数字。
     * - 0x00000006 REG_LINK   指向注册表项的符号链接。
     * - 0x00000007 REG_MULTI_SZ      MS-RRP
     * - 0x00000008 REG_RESOURCE_LIST 设备驱动程序资源列表
     * - 0x0000000B REG_QWORD 64 位数字
     * @param dataSize 传址 大小
     * @return true
     * @return false
     */
    bool getValueStat(HKEY hKey, string subKey, string key, DWORD &pValueType, DWORD &pDataSize)
    {

        pValueType = 0x00000000;
        pDataSize = 0;
        HKEY subHKey;
        if (RegOpenKeyExA(hKey, subKey.c_str(), 0, KEY_ALL_ACCESS, &subHKey) == ERROR_SUCCESS)
        {
            _defined_auto_free_HKey(subHKey);

            DWORD valueType;
            DWORD dataSize = 0;

            // 第一次调用 RegQueryValueEx 获取值的大小，放入 dataSize 变量中
            if (RegQueryValueExA(subHKey, key.c_str(), nullptr, &valueType, nullptr, &dataSize) == ERROR_SUCCESS)
            {
                pValueType = valueType + 0;
                pDataSize = dataSize + 0;
                return true;
            }
            else
            {
                return false;
            }
        }
        else
        {
            return false;
        }
    }

    /**
     * @brief 获取值类型与值路径
     *
     * @param hKey 根
     * @param path 路径
     * @param valueType 传址 DW
     * - 0x00000000 REG_NONE 未定义
     * - 0x00000001 REG_SZ 字符串
     * - 0x00000002 REG_EXPAND_SZ 未展开引用的字符串 例如“%PATH%”
     * - 0x00000003 REG_BINARY 二进制
     * - 0x00000004 REG_DWORD / REG_DWORD_LITTLE_ENDIAN 32 位数字
     * - 0x00000005 REG_DWORD_BIG_ENDIAN 大端格式的 32 位数字。
     * - 0x00000006 REG_LINK   指向注册表项的符号链接。
     * - 0x00000007 REG_MULTI_SZ      MS-RRP
     * - 0x00000008 REG_RESOURCE_LIST 设备驱动程序资源列表
     * - 0x0000000B REG_QWORD 64 位数字
     * @param dataSize 传址 大小
     * @return true
     * @return false
     */
    chValueStat getValueStat(HKEY hKey, string subKey, string key)
    {
        chValueStat result = {
            0, 0, 0};
        HKEY subHKey;
        if (RegOpenKeyExA(hKey, subKey.c_str(), 0, KEY_ALL_ACCESS, &subHKey) == ERROR_SUCCESS)
        {
            _defined_auto_free_HKey(subHKey);

            DWORD valueType;
            DWORD dataSize = 0;

            // 第一次调用 RegQueryValueEx 获取值的大小，放入 dataSize 变量中
            if (RegQueryValueExA(subHKey, key.c_str(), nullptr, &valueType, nullptr, &dataSize) == ERROR_SUCCESS)
            {
                result.exists = true;
                result.size = dataSize;
                result.type = valueType;
            }
        }
        return result;
    }

    /**
     * @brief 设置内容 自识别或者自定义
     *
     * @tparam T 多种支持格式
     * @param hKey 根
     * @param subKey 目录
     * @param key 键
     * @param valueData 值
     * @param retype 指定类型 默认自识别
     * - 0x00000000 REG_NONE 未定义
     * - 0x00000001 REG_SZ 字符串
     * - 0x00000002 REG_EXPAND_SZ 未展开引用的字符串 例如“%PATH%”
     * - 0x00000003 REG_BINARY 二进制
     * - 0x00000004 REG_DWORD / REG_DWORD_LITTLE_ENDIAN 32 位数字
     * - 0x00000005 REG_DWORD_BIG_ENDIAN 大端格式的 32 位数字。
     * - 0x00000006 REG_LINK   指向注册表项的符号链接。
     * - 0x00000007 REG_MULTI_SZ      MS-RRP
     * - 0x00000008 REG_RESOURCE_LIST 设备驱动程序资源列表
     * - 0x0000000B REG_QWORD 64 位数字
     * @return true
     * @return false
     */
    template <typename T>
    bool setRegistrValue(HKEY hKey, string subKey, string key, const T &valueData, DWORD retype = 0)
    {
        bool result = false;

        static_assert(
            is_integral<T>::value ||
                is_same_v<T, string> ||
                is_same_v<T, vector<unsigned char>>,
            "Unsupported type preset escape (不支持的类型预设转义)");

        try
        {
            HKEY hSubKey;
            DWORD dwDisposition;
            DWORD is_open = ::RegCreateKeyExA(hKey, subKey.c_str(), 0, nullptr, REG_OPTION_NON_VOLATILE, KEY_WRITE, nullptr, &hSubKey, &dwDisposition);
            _defined_auto_free_HKey(hSubKey);
            // 尝试创建或者打开父键
            if (is_open == ERROR_SUCCESS)
            {

                // 数字小于 64 写入DWORD
                if constexpr (_define_is_int32bit(T))
                {
                    DWORD newData = ((DWORD)valueData) + 0;
                    is_open = ::RegSetValueExA(hSubKey, key.c_str(), 0, (retype != 0 ? retype : REG_DWORD), reinterpret_cast<const BYTE *>(newData), sizeof(DWORD));
                }
                // 写入64位数字
                else if constexpr (_define_is_int64bit(T))
                {
                    long long newData = ((long long)valueData) + 0;
                    is_open = ::RegSetValueExA(hSubKey, key.c_str(), 0, (retype != 0 ? retype : REG_QWORD), reinterpret_cast<const BYTE *>(newData), sizeof(int64_t));
                }
                // 写入文本
                else if constexpr (is_same_v<T, string>)
                {
                    is_open = ::RegSetValueExA(hSubKey, key.c_str(), 0, (retype != 0 ? retype : REG_SZ), reinterpret_cast<const BYTE *>(valueData.c_str()), static_cast<DWORD>(string(valueData).size() * sizeof(char)));
                }
                // 写入二进制
                else if constexpr (is_same_v<T, vector<BYTE>>)
                {
                    is_open = ::RegSetValueExA(hSubKey, key.c_str(), 0, (retype != 0 ? retype : REG_BINARY), reinterpret_cast<const BYTE *>(valueData.data()), static_cast<DWORD>(valueData.size() * sizeof(char)));
                }

                else
                {
                    is_open = 999;
                }

                result = is_open == ERROR_SUCCESS;
                return result;
            }
        }
        catch (const exception &e)
        {
            return result;
        }

        return result;
    }

    /**
     * @brief 获取指定的值
     * ? string  ->  getRegistrValue <string> (hKey, subKey,key);
     * ? bin  ->  getRegistrValue< vector<BYTE> >(hKey, subKey,key);
     * ? int  -> getRegistrValue <int> (hKey, subKey,key);
     * ? REG_RESOURCE_REQUIREMENTS_LIST  ->  getRegistrValue< vector<BYTE> >(hKey, subKey,key);
     * ? REG_RESOURCE_LIST  ->  getRegistrValue< vector<BYTE> >(hKey, subKey,key);
     * - 读取为未转义变量
     * ? string -> getRegistrValue <string> (hKey, 'subKey','key',REG_EXPAND_SZ);
     * -
     * @param hKey
     * @param subKey
     * @param key
     * @return T <int , int8_t , int32_t , >
     */
    template <typename T>
    T getRegistrValue(HKEY hKey, string subKey, string key, DWORD retype = 0)
    {
        T result_default = {0};
        static_assert(
            is_integral<T>::value ||
                is_same_v<T, string> ||
                is_same_v<T, vector<unsigned char>>,
            "Unsupported type preset escape (不支持的类型预设转义)");

        DWORD is_open;
        DWORD pValueType;
        DWORD pDataSize;
        bool is_ok = false;
        HKEY subHKey = 0;
        //hmc_EnableShutDownPriv();
        hmc_registr::chValueStat data = getValueStat(hKey, subKey, key);
        is_ok = data.exists;
        pDataSize = data.size;
        pValueType = data.type;

        is_open = ::RegOpenKeyExA(hKey, subKey.c_str(), 0, KEY_ALL_ACCESS, &subHKey) == ERROR_SUCCESS;
        // 智能关闭指针
        _defined_auto_free_HKey(subHKey);

        // 处理32位数字
        if constexpr (_define_is_int32bit(T))
        {
            int32_t result = 0;

            // 条件隐式转换匹配 不符合直接跳出
            _define_if_to_break(_EQ_REG_TYPE(pValueType, retype), result);

            DWORD type = retype == 0 ? REG_DWORD : retype;

            long long value_data = 0;
            if (type == 0)
                type = REG_DWORD;

            if (::RegQueryValueExA(subHKey, key.c_str(), 0, &type, reinterpret_cast<BYTE *>(&value_data), &pDataSize) == ERROR_SUCCESS)
            {
                result = (int32_t)value_data;
                return result;
            }
            return result;
        }
        // 处理64位数字
        else if constexpr (_define_is_int64bit(T))
        {
            long long result = 0;

            // 条件隐式转换匹配 不符合直接跳出
            _define_if_to_break(_EQ_REG_TYPE(pValueType, retype), result);
            DWORD type = retype == 0 ? REG_QWORD : retype;

            if (::RegQueryValueExA(subHKey, key.c_str(), 0, &type, reinterpret_cast<BYTE *>(&result), &pDataSize) == ERROR_SUCCESS)
            {
                return result;
            }
        }
        // 处理文本型
        else if constexpr (is_same_v<T, string>)
        {
            string result = string();

            // 条件隐式转换匹配 不符合直接跳出
            _define_if_to_break(_EQ_REG_TYPE(pValueType, retype), result);

            DWORD type = retype == 0 ? pValueType : retype;

            vector<BYTE> value_data(pDataSize);

            if (::RegQueryValueExA(subHKey, key.c_str(), 0, &type, reinterpret_cast<BYTE *>(value_data.data()), &pDataSize) == ERROR_SUCCESS)
            {
                result.resize(pDataSize);
                for (size_t i = 0; i < pDataSize; i++)
                {
                    result[i] = value_data[i];
                }
            }

            return result;
        }
        // 处理二进制
        else if constexpr (is_same_v<T, vector<BYTE>>)
        {
            vector<BYTE> value_data(pDataSize);

            _define_if_to_break(_EQ_REG_TYPE(pValueType, retype), value_data);

            DWORD type = retype == 0 ? REG_BINARY : retype;

            ::RegQueryValueExA(subHKey, key.c_str(), 0, &type, reinterpret_cast<BYTE *>(value_data.data()), &pDataSize);

            return value_data;
        }

        return result_default;
    }

    /**
     * @brief 获取单条数据并返回类型与数据
     *
     * @param hKey
     * @param subKey
     * @param key
     * @return chWalkValueItmeCout
     */
    chWalkItme getRegistrAnyValue(HKEY hKey, string subKey, string key)
    {
        chWalkItme walkValueItmeCout;
        walkValueItmeCout.dirPath = subKey;
        walkValueItmeCout.vkey = key;
        walkValueItmeCout.isDir = false;
        walkValueItmeCout.size = 0;
        walkValueItmeCout.time = 0;
        walkValueItmeCout.type = 0;
        walkValueItmeCout.root = hKey;
        walkValueItmeCout.value = {0};
        walkValueItmeCout.is_value = false;
        HKEY open_hkey = nullptr;
        _defined_auto_free_HKey(open_hkey);

        DWORD is_open;
        DWORD pValueType;
        DWORD pDataSize;
        bool is_ok = false;
        is_ok = getValueStat(hKey, subKey, key, pValueType, pDataSize);

        is_open = ::RegOpenKeyExA(hKey, subKey.c_str(), 0, KEY_ALL_ACCESS, &open_hkey) == ERROR_SUCCESS;
        vector<BYTE> value_data(pDataSize);

        if (!is_ok || !is_open)
            return walkValueItmeCout;
        if (ERROR_SUCCESS == ::RegQueryValueExA(open_hkey, key.c_str(), 0, REG_NONE, reinterpret_cast<BYTE *>(value_data.data()), &pDataSize))
        {
            walkValueItmeCout.value.clear();
            walkValueItmeCout.value.resize(value_data.size());
            walkValueItmeCout.size = value_data.size();
            walkValueItmeCout.type = pValueType + 0;
            walkValueItmeCout.is_value = true;
            for (size_t i = 0; i < value_data.size(); i++)
            {
                walkValueItmeCout.value[i] = value_data[i];
            }
        };
        value_data.clear();
        return walkValueItmeCout;
    }

    /**
     * @brief 判断是否存在此key
     *
     * @param hKey
     * @param subKey
     * @param key
     * @return true
     * @return false
     */
    bool hasRegistrKey(HKEY hKey, string subKey, string key)
    {
        bool result = false;

        DWORD pValueType;
        DWORD pDataSize;
        result = getValueStat(hKey, subKey, key, pValueType, pDataSize);
        return result;
    }

    /**
     * @brief 判断是否存在此key
     *
     * @param hKey
     * @param subKey
     * @param key
     * @return true
     * @return false
     */
    bool hasRegistrDir(HKEY hKey, string subKey)
    {
        bool result = false;
        HKEY hTestKey;
        DWORD openResult = RegOpenKeyExA(hKey, subKey.c_str(), 0, KEY_ALL_ACCESS, &hTestKey);
        _defined_auto_free_HKey(hTestKey);

        switch (openResult)
        {
        case ERROR_SUCCESS:
            return true;
        case ERROR_FILE_NOT_FOUND:
            return false;
        }
        return result;
    }

    /**
     * @brief 删除指定的值
     *
     * @param hKey
     * @param subKey
     * @param key
     * @return true
     * @return false
     */
    bool removeRegistrValue(HKEY hKey, string subKey, string key)
    {
        HKEY open_hkey = nullptr;
        _defined_auto_free_HKey(open_hkey);
        if (ERROR_SUCCESS == ::RegOpenKeyExA(hKey, subKey.c_str(), 0, KEY_ALL_ACCESS, &open_hkey))
        {
            return ::RegDeleteValueA(open_hkey, key.c_str()) == ERROR_SUCCESS;
        }

        return false;
    }

    /**
     * @brief 删除注册表值树
     *
     * @param hKey
     * @param subKey
     * @return true
     * @return false
     */
    bool removeRegistrTree(HKEY hKey, string subKey, string DirName)
    {
        HKEY open_hkey = nullptr;
        _defined_auto_free_HKey(open_hkey);
        if (ERROR_SUCCESS == ::RegOpenKeyExA(hKey, subKey.c_str(), 0, KEY_ALL_ACCESS, &open_hkey))
        {
            RegDeleteTreeA(open_hkey, DirName.c_str());
        }
        return hasRegistrDir(hKey, subKey + "\\" + DirName) == false;
    }

    /**
     * @brief 删除指定文件夹
     *
     * @param hKey
     * @param subKey
     * @param tree 是否删除所有
     * @return true
     * @return false
     */
    bool removeRegistrDir(HKEY hKey, string keyPath, bool tree = false)
    {
        HKEY open_hkey = nullptr;

        if (tree)
        {
            vector<string> keys = _lib_splitString(keyPath);
            if (keys.size() == 0)
                return false;
            string dirName = keys[keys.size() - 1];
            keys.pop_back();
            return removeRegistrTree(hKey, _lib_joinString(keys, "\\"), dirName);
        }

        LONG result = ::RegOpenKeyExA(HKEY_CURRENT_USER, keyPath.c_str(), 0, KEY_ALL_ACCESS, &open_hkey);
        _defined_auto_free_HKey(open_hkey);

        if (result == ERROR_SUCCESS)
        {
            result = ::RegDeleteKeyExA(HKEY_CURRENT_USER, keyPath.c_str(), KEY_WOW64_64KEY, 0);
            return (result == ERROR_SUCCESS) || (::RegDeleteKeyExA(HKEY_CURRENT_USER, keyPath.c_str(), KEY_WOW64_32KEY, 0) == ERROR_SUCCESS);
        }
        else
        {
            return false;
        }
        return false;
    }

    /**
     * @brief 创建文件夹
     *
     * @param hKey
     * @param keyPath
     * @return true
     * @return false
     */
    bool createRegistrDir(HKEY hKey, string keyPath)
    {
        HKEY open_hkey = nullptr;
        DWORD dwOptions = REG_OPTION_NON_VOLATILE;
        DWORD dwDisposition;
        long resulte = RegCreateKeyExA(hKey, keyPath.c_str(), 0, NULL,
                                       dwOptions, KEY_WRITE, NULL, &open_hkey, &dwDisposition);
        if (resulte != ERROR_SUCCESS)
        {
            return false;
        }
        else
        {
            switch (dwDisposition)
            {
            case REG_OPENED_EXISTING_KEY:
                return true;
            case REG_CREATED_NEW_KEY:
                return true;
            }
        }
    }

    /**
     * @brief 复制指定的目录到指定目录
     *
     * @param hKey
     * @param sourcePath
     * @param toPath
     * @return true
     * @return false
     */
    bool copyRegistrDir(HKEY hKey, string sourcePath, string toPath)
    {
        HKEY sourceHKey = nullptr;
        HKEY toHKey = nullptr;
        DWORD dwDisposition;

        _defined_auto_free_HKey(sourceHKey);
        shared_ptr<void> close_toHKey(nullptr, [&](void *)
                                      {
        if (toHKey != nullptr) {
            ::RegCloseKey(toHKey);
            toHKey = nullptr;
        } });

        if (ERROR_SUCCESS != ::RegOpenKeyExA(hKey, sourcePath.c_str(), 0, KEY_ALL_ACCESS, &sourceHKey))
        {
            return false;
        }

        if (ERROR_SUCCESS != ::RegCreateKeyExA(hKey, toPath.c_str(), 0, nullptr, REG_OPTION_NON_VOLATILE, KEY_ALL_ACCESS, nullptr, &toHKey, &dwDisposition))
        {
            return false;
        }

        return (ERROR_SUCCESS == ::RegCopyTreeA(sourceHKey, nullptr, toHKey));
    }

    /**
     * @brief 获取目录表中的键
     *
     * @param hKey   根目录
     * @param keyPath  获取目录路径
     * @param filterType 过滤类型
     * - all  REG_NONE
     * - string  REG_SZ|REG_EXPAND_SZ|REG_LINK
     * - number  REG_DWORD|REG_QWORD|REG_DWORD_BIG_ENDIAN
     * - bin     REG_BINARY|REG_DWORD_LITTLE_ENDIAN|REG_DWORD_BIG_ENDIAN|REG_RESOURCE_LIST|REG_RESOURCE_REQUIREMENTS_LIST|REG_FULL_RESOURCE_DESCRIPTOR
     * @return true
     * @return false
     */
    template <typename... Args>
    vector<chWalkItme> walkRegistrDir(HKEY hKey, string keyPath, bool addValue = false, Args... typeFlag)
    {

        vector<chWalkItme> result;
        try
        {
            HKEY hTestKey;
            _defined_auto_free_HKey(hTestKey);
            vector<string> keylist;
            vector<string> dirlist;
            set<DWORD> typeFlagList;
            DWORD temp[] = {typeFlag...};
            for (size_t i = 0; i < sizeof(temp) / sizeof(temp[0]); i++)
                typeFlagList.insert((DWORD)temp[i]);

            if (RegOpenKeyExA(hKey, keyPath.c_str(),
                              0,
                              KEY_ALL_ACCESS,
                              &hTestKey) == ERROR_SUCCESS)
            {
                _lib_EnumRegistrKeyQuery(hTestKey, dirlist, keylist);
            }

            {
                // 枚举并获取信息
                for (size_t i = 0; i < keylist.size(); i++)
                {
                    chWalkItme walkItemCout;
                    string key = keylist[i];
                    walkItemCout.vkey = key;
                    walkItemCout.dirPath = keyPath;
                    walkItemCout.time = 0;
                    walkItemCout.isDir = false;
                    walkItemCout.root = hKey;
                    walkItemCout.value = {0};

                    DWORD pValueType;
                    DWORD pDataSize;
                    getValueStat(hKey, keyPath, key, pValueType, pDataSize);
                    if (typeFlagList.find(REG_NONE) != typeFlagList.end() || typeFlagList.find(pValueType) != typeFlagList.end())
                    {

                        walkItemCout.size = pDataSize;
                        walkItemCout.type = pValueType;
                        if (addValue)
                        {
                            walkItemCout.value = getRegistrAnyValue(hKey, keyPath, key).value;
                        }

                        result.push_back(walkItemCout);
                    }
                }

                // 枚举并获取信息
                if (typeFlagList.find(REG_NONE) != typeFlagList.end())
                    for (size_t i = 0; i < dirlist.size(); i++)
                    {
                        chWalkItme walkItemCout;
                        string dir = dirlist[i];
                        chQueryDirStat chqlist = getRegistrDirStat(hKey, string(keyPath).append("\\").append(dir));
                        walkItemCout.vkey = dir;
                        walkItemCout.dirPath = keyPath;
                        walkItemCout.time = chqlist.LastWriteTime;
                        walkItemCout.isDir = true;
                        walkItemCout.root = hKey;
                        walkItemCout.size = 0;
                        walkItemCout.type = 0;
                        walkItemCout.value = {0};
                        result.push_back(walkItemCout);
                    }
            }
        }
        catch (const exception &e)
        {
        }
        return result;
    }

};

#endif