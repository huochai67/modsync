#include <iostream>
#include <vector>
#include <map>

#include <cpr/cpr.h>

#include "common.hpp"

#include "gui_service.h"

#define U8TEXT(_S) (const char*)u8##_S

typedef std::map<std::string, modsync::ModInfo> ModMap;
template<typename T>
std::map<std::string, T> Vector2Map(std::vector<T> mis)
{
	std::map<std::string, T> map;
	for (auto& x : mis)
	{
		for (auto i = 0; i < x.md5.size(); ++i)
			x.md5[i] = std::toupper(x.md5[i]);
		map.insert({ x.md5, x });
	}
	return std::move(map);
}

GuiService gs;
struct _global
{
	std::string changelog = "";
	std::wstring datadirectory = std::filesystem::current_path().wstring() + L"/../.minecraft";
	modsync::uinfo info;
	ModMap modsmap_remote;
	bool btnModSyncActive = true;
	bool btnKeyMapActive = true;
	bool btnServerListActive = true;
	bool isUpdate = false;
} global;
void DisableAllButton()
{
	global.isUpdate = true;
	global.btnModSyncActive = false;
	global.btnKeyMapActive = false;
	global.btnServerListActive = false;
}
void EnableAllButton()
{
	global.isUpdate = false;
	global.btnModSyncActive = true;
	global.btnKeyMapActive = true;
	global.btnServerListActive = true;
}

void updateRemoteConfig()
{
	auto r = cpr::Get(cpr::Url("https://msupdate-1255639557.cos.ap-guangzhou.myqcloud.com/info.json"));
	global.info = nlohmann::json::parse(r.text);

	if (global.info.hasModList)
	{
		auto r2 = cpr::Get(cpr::Url(global.info.modListUrl));
		std::vector<modsync::ModInfo> vecremote = nlohmann::json::parse(r2.text);
		global.modsmap_remote = Vector2Map(vecremote);
	}
}

class DownloadFormTask
{
public:
	DownloadFormTask(const std::string& url_, const std::filesystem::path& path_)
		: m_url(url_)
		, m_path(path_)
	{
		auto parent_path = this->m_path.parent_path();
		std::filesystem::create_directories(parent_path);

		m_file.open(path_, std::ios::binary);

		m_rep = std::make_shared<cpr::AsyncResponse>(cpr::GetAsync(cpr::Url{ url_ }
			, cpr::ProgressCallback([this](cpr::cpr_off_t downloadTotal, cpr::cpr_off_t downloadNow, cpr::cpr_off_t uploadTotal, cpr::cpr_off_t uploadNow, intptr_t userdata) {
				this->m_progress = (float)downloadNow / (float)downloadTotal;
				return true;
				})
			, cpr::WriteCallback([this](std::string data, intptr_t userdata) {
					this->m_file.write(data.c_str(), data.size());
					return true;
				})
					));
	};
	~DownloadFormTask() {
		if (this->m_file.is_open())
			this->m_file.close();
	};

	inline bool IsVaild() { return this->m_rep->valid(); };
	inline float* GetProgressPtr() { return &this->m_progress; };
	inline cpr::Response Get() { return this->m_rep->get(); };
	inline std::filesystem::path& GetPath() { return this->m_path; };

private:
	std::string  m_url;
	std::filesystem::path m_path;
	float m_progress = -1.f;

	std::ofstream m_file;

	std::shared_ptr<cpr::AsyncResponse> m_rep = nullptr;
};
typedef std::shared_ptr<DownloadFormTask> DownloadFormTaskPtr;

class DownloadForm
{
public:
	DownloadForm() {};

	void Run(const ImVec2& size)
	{
		ImGui::BeginChild("Scrolling Area", size, true, ImGuiWindowFlags_AlwaysVerticalScrollbar);
		std::vector<std::list<DownloadFormTaskPtr>::const_iterator> vecRemove;
		if (!this->m_listTasks.size())
			EnableAllButton();
		for (auto iter = this->m_listTasks.begin(); iter != this->m_listTasks.end(); ++iter)
		{
			auto& ptr = *iter;
			auto progress = *ptr->GetProgressPtr() * 100;
			if (progress == 100.f)
			{
				m_numTaskFinished++;
				vecRemove.push_back(iter);
			}

			ImGui::PushStyleColor(ImGuiCol_Text, IM_COL32(0, 255, 0, 255));
			ImGui::Text(std::format("{:.0f}% ", progress).c_str());
			ImGui::PopStyleColor();
			ImGui::SameLine();
			auto tmp = ptr->GetPath().wstring();
			ImGui::Text(utf16le_to_utf8(std::u16string(tmp.begin(), tmp.end())).c_str());
		}
		ImGui::EndChild();

		for (auto& x : vecRemove)
			m_listTasks.erase(x);
	}


	void AddTask(const std::string& url, const std::filesystem::path& path)
	{
		m_listTasks.push_back(std::make_shared<DownloadFormTask>(url, path));
		this->m_numTaskTotal++;
	}
	void AddTask(const std::wstring& url, const std::filesystem::path& path)
	{
		this->AddTask(utf16le_to_utf8(std::u16string(url.begin(), url.end())), path);
	}

	inline void ClearTaskNum() { this->m_numTaskFinished = 0; this->m_numTaskTotal = 0; }

	inline float GetTotalProgress() {
		return m_numTaskFinished ? float(m_numTaskFinished) / float(m_numTaskTotal) : 0;
	};
private:
	std::list<DownloadFormTaskPtr> m_listTasks;

	int m_numTaskFinished = 0;
	int m_numTaskTotal = 0;

} downloadform_;

void SyncMOD()
{
	updateRemoteConfig();

	auto modspath = global.datadirectory + L"/mods/";
	modspath = std::filesystem::absolute(modspath);
	std::filesystem::create_directories(modspath);

	ModMap mmlocal;
	ModMap modsmap_remoteCopy = global.modsmap_remote;

	std::vector<std::filesystem::path> paths;
	modsync::dir_foreach(modspath, &paths);
	for (auto& x : paths)
	{
		auto mi = modsync::GetModInfo(modspath, x);
		mmlocal.insert({ mi.md5, mi});
	}

	for (auto& x : mmlocal)
		if (auto iter = modsmap_remoteCopy.find(x.first); iter == modsmap_remoteCopy.end())
			std::filesystem::remove(modspath + x.second.path);
		else modsmap_remoteCopy.erase(iter);

	for (auto& x : modsmap_remoteCopy)
	{
		//download
		auto& mi = x.second;
		//auto tmppath = utf8_to_utf16le(mi.path);
		//auto wpath = modspath + std::wstring(tmppath.begin(), tmppath.end());
		auto wpath = modspath + mi.path;
		downloadform_.AddTask(mi.url, wpath);
	}
}

void render()
{
	ImGuiIO& io = ImGui::GetIO();
	auto ds = ImGui::GetIO().DisplaySize;
	ImGuiWindowFlags window_flags = ImGuiWindowFlags_NoMove | ImGuiWindowFlags_NoResize | ImGuiWindowFlags_NoTitleBar;
#ifdef IMGUI_HAS_VIEWPORT
	ImGuiViewport* viewport = ImGui::GetMainViewport();
	ImGui::SetNextWindowPos(viewport->GetWorkPos());
	ImGui::SetNextWindowSize(viewport->GetWorkSize());
	ImGui::SetNextWindowViewport(viewport->ID);
#else 
	ImGui::SetNextWindowPos(ImVec2(0.0f, 0.0f));
	ImGui::SetNextWindowSize(ds);
#endif
	ImGui::Begin("Main Windows", (bool*)true, window_flags);

	//滑动界面
	auto sszie = ImVec2(ds.x, ds.y - 140);
	if (!global.isUpdate)
	{
		ImGui::BeginDisabled();
		ImGui::InputTextMultiline("##CHANGELOG", (char*)global.changelog.c_str(), global.changelog.size() + 1, sszie);
		ImGui::EndDisabled();
	}
	else
	{
		downloadform_.Run(sszie);
	}

	//进度条
	ImGui::BeginDisabled();
	auto progress_ = downloadform_.GetTotalProgress() * 100;
	ImGui::SliderFloat("##PROGRESSBAR", &progress_, 0.0f, 100.0f, "%.1f", ImGuiSliderFlags_None);
	ImGui::EndDisabled();

	//按钮栏
	ImGui::BeginDisabled(!global.btnModSyncActive);
	if (ImGui::Button(U8TEXT("更新mod")))
	{
		DisableAllButton();
		downloadform_.ClearTaskNum();
		SyncMOD();
	}
	ImGui::EndDisabled();
	ImGui::SameLine();
	ImGui::BeginDisabled(!global.btnKeyMapActive);
	if (ImGui::Button(U8TEXT("更新按键设定")))
	{
		DisableAllButton();
		downloadform_.AddTask(global.info.optionUrl, global.datadirectory + L"/options.txt");
	}
	ImGui::EndDisabled();
	ImGui::SameLine();
	ImGui::BeginDisabled(!global.btnServerListActive);
	if (ImGui::Button(U8TEXT("更新服务器列表")))
	{
		DisableAllButton();
		downloadform_.AddTask(global.info.serverListUrl, global.datadirectory + L"/servers.dat");
	}
	ImGui::EndDisabled();

	//底部状态
	ImGui::Text("Status: "); ImGui::SameLine();
	if (progress_ == 100.f || progress_ == 0.f)
	{
		ImGui::PushStyleColor(ImGuiCol_Text, IM_COL32(0, 255, 0, 255));
		ImGui::Text("OK");
		ImGui::PopStyleColor();
	}
	else
	{
		ImGui::PushStyleColor(ImGuiCol_Text, IM_COL32(255, 0, 0, 255));
		ImGui::Text("Working");
		ImGui::PopStyleColor();
	}
	ImGui::Text("average %.3f ms/frame (%.1f FPS)", 1000.0f / io.Framerate, io.Framerate);
	if (ImGui::Selectable("MODSync Client Version: 2.0", false, ImGuiSelectableFlags_AllowDoubleClick))
		if (ImGui::IsMouseDoubleClicked(0))
		{
#ifdef _WIN32 
			// Note: executable path must use  backslashes! 
			::ShellExecuteA(NULL, "open", "https://sfwork.coding.net/public/modsync/modsync/git/files", NULL, NULL, SW_SHOWDEFAULT);
#else		
			char command[256]; 
			snprintf(command, 256, "%s \"%s\"",
				open_executable, path);
			system(command);
#endif
		}
	ImGui::End();
}

int main() {
	gs.ReSize(745, 360);
	static bool done = false;
	gs.SetRenderCallBack(&render);
	updateRemoteConfig();

	auto r = cpr::Get(cpr::Url(global.info.changelogUrl));
	global.changelog =  r.text;

	gs.SetTitle(global.info.title);
	while (!done)
		gs.Run(&done);
	return 0;
}

#ifdef WIN32
int WINAPI wWinMain(HINSTANCE hInstance, HINSTANCE hPrevInstance, PWSTR pCmdLine, int nCmdShow)
{
	main();
	return 0;
}
#endif // WIN32
