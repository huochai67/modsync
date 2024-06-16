#include <string>
#include <vector>
#include <filesystem>
#include <locale>
#include <codecvt>

#include <openssl/evp.h>
#include <openssl/md5.h>

#include "json.hpp"

#include "utfconvert.h"

namespace modsync
{
	struct uinfo {
		std::string baseUrl;
		std::string serverListUrl;
		std::string changelogUrl;
		std::string optionUrl;
		std::string modListUrl;
		std::string title;
		bool forceSyncServerList = false;
		bool hasServerList = false;
		bool hasChangelog = false;
		bool hasOption = false;
		bool hasModList = false;
	};

	void to_json(nlohmann::json& j, const uinfo& _uinfo) {
		j = nlohmann::json{
			{"baseUrl", _uinfo.baseUrl}
		,{"serverListUrl", _uinfo.serverListUrl}
		,{"changelogUrl", _uinfo.changelogUrl}
		,{"optionUrl", _uinfo.optionUrl}
		,{"modListUrl", _uinfo.modListUrl}
		,{"title", _uinfo.title}
		,{"forceSyncServerList", _uinfo.forceSyncServerList}
		,{"hasServerList", _uinfo.hasServerList}
		,{"hasChangelog", _uinfo.hasChangelog}
		,{"hasOption", _uinfo.hasOption}
		,{"hasModList", _uinfo.hasModList} };
	}

	void from_json(const nlohmann::json& j, uinfo& info) {
		j["baseUrl"].get_to<std::string>(info.baseUrl);
		j["serverListUrl"].get_to<std::string>(info.serverListUrl);
		j["changelogUrl"].get_to<std::string>(info.changelogUrl);
		j["optionUrl"].get_to<std::string>(info.optionUrl);
		j["modListUrl"].get_to<std::string>(info.modListUrl);
		j["title"].get_to<std::string>(info.title);
		j["forceSyncServerList"].get_to<bool>(info.forceSyncServerList);
		j["hasServerList"].get_to<bool>(info.hasServerList);
		j["hasChangelog"].get_to<bool>(info.hasChangelog);
		j["hasOption"].get_to<bool>(info.hasOption);
		j["hasModList"].get_to<bool>(info.hasModList);
	}

	struct ModInfo {
		std::wstring path;
		std::wstring url;
		std::string md5;
		size_t size;
	};

	void to_json(nlohmann::json& j, const ModInfo& _modinfo) {
		j = nlohmann::json{
			{"path", _modinfo.path}
		,{"url", _modinfo.url}
		,{"md5", _modinfo.md5}
		,{"size", _modinfo.size} };
	}

	void from_json(const nlohmann::json& j, ModInfo& _modinfo) {
		j["path"].get_to<std::wstring>(_modinfo.path);
		j["url"].get_to<std::wstring>(_modinfo.url);
		j["md5"].get_to<std::string>(_modinfo.md5);
		j["size"].get_to<size_t>(_modinfo.size);
	}

	void dir_foreach(const std::filesystem::path& path, std::vector<std::filesystem::path>* paths) {
		for (auto& x : std::filesystem::directory_iterator{ path })
		{
			if (x.is_directory())
				dir_foreach(x.path().string().c_str(), paths);
			else
				paths->push_back(x.path());
		}
	}

	ModInfo GetModInfo(const std::filesystem::path& dirpath, const std::filesystem::path& filepath, const std::wstring& url = L"") {
		EVP_MD_CTX* evpCtx = EVP_MD_CTX_new();
		EVP_DigestInit_ex(evpCtx, EVP_md5(), NULL);

		std::ifstream ifile(filepath, std::ios::in | std::ios::binary);
		assert(ifile.is_open());
		ifile.seekg(0, ifile.end);
		size_t size = ifile.tellg();
		ifile.seekg(0, ifile.beg);

		char buffer[1024];
		while (!ifile.eof())
		{
			ifile.read(buffer, 1024);
			int length = ifile.gcount();
			if (length)
				EVP_DigestUpdate(evpCtx, buffer, length);
		}
		ifile.close();

		unsigned char digest[MD5_DIGEST_LENGTH];
		unsigned int outsize;
		EVP_DigestFinal_ex(evpCtx, digest, &outsize);
		EVP_MD_CTX_free(evpCtx);

		std::string strmd5;
		strmd5.resize(MD5_DIGEST_LENGTH * 2);
		for (auto i = 0; i < 16; i++)
			sprintf(&strmd5[i * 2], "%02X", digest[i]);

		auto strpath = filepath.wstring();
		auto psize = strpath.size() - dirpath.wstring().size();

		auto cpath = strpath.substr(dirpath.wstring().size(), psize);
		auto pos = cpath.find('\\');
		while (pos != std::string::npos) {
			cpath[pos] = '/';
			pos = cpath.find('\\');
		}
		return ModInfo{ cpath, url + cpath, strmd5, size };
	}
}