#include <iostream>
#include <vector>
#include <map>

#include <cpr/cpr.h>

#include "common.hpp"

#include "gui_service.h"

#define U8TEXT(_S) (const char*)u8##_S

struct _global
{
	std::wstring datadirectory = std::filesystem::current_path().wstring() + L"/data/";
	modsync::uinfo info;
} global;

void writefile(const std::wstring& path, const std::string& data, const std::string em)
{
	std::fstream file;
	file.open(path, std::ios::out, std::ios::trunc);
	if (file.is_open())
	{
		file << data;
		file.close();
	}
	return;
}

static char url[256];
static char title[256];
void reloadconfig()
{
	auto path = std::filesystem::current_path().string() + "/data/info.json";
	if (!std::filesystem::exists(path))
		return;

	std::fstream file;
	file.open(path, std::ios::in);
	if (file.is_open())
	{
		nlohmann::json json;
		json << file;
		file.close();
		global.info = json;
	}
	::memcpy(url, global.info.baseUrl.c_str(), global.info.baseUrl.size());
	::memcpy(title, global.info.title.c_str(), global.info.title.size());
	return;
}

void genconfig()
{
	auto dirname = L"data/mods";
	auto datapath = global.datadirectory + dirname;
	std::vector<std::filesystem::path> mods;
	modsync::dir_foreach(datapath, &mods);

	std::vector<modsync::ModInfo> modinfos;
	for (auto& x : mods)
		modinfos.push_back(modsync::GetModInfo(datapath, x.wstring(), std::wstring(global.info.baseUrl.begin(), global.info.baseUrl.end()) + dirname + L"/"));
	writefile(global.datadirectory + L"modslist.json", nlohmann::json(modinfos).dump(), "写入mod信息失败");

	modsync::uinfo newinfo;
	newinfo.hasModList = true;
	if (std::filesystem::exists(global.datadirectory + L"data/options.txt"))
		newinfo.hasOption = true;
	if (std::filesystem::exists(global.datadirectory + L"data/servers.dat"))
		newinfo.hasServerList = true;
	if (std::filesystem::exists(global.datadirectory + L"changelog.txt"))
		newinfo.hasChangelog = true;
	newinfo.changelogUrl = global.info.baseUrl + "changelog.txt";
	newinfo.serverListUrl = global.info.baseUrl + "data/servers.dat";
	newinfo.optionUrl = global.info.baseUrl + "data/options.txt";
	newinfo.modListUrl = global.info.baseUrl + "modslist.json";
	newinfo.baseUrl = global.info.baseUrl;
	newinfo.title = global.info.title;
	writefile(global.datadirectory + L"info.json", nlohmann::json(newinfo).dump(), "写入配置失败");

	return;
}

void render()
{
	ImGuiIO& io = ImGui::GetIO();
	ImGuiWindowFlags window_flags = ImGuiWindowFlags_NoTitleBar | ImGuiWindowFlags_NoMove | ImGuiWindowFlags_NoResize;
#ifdef IMGUI_HAS_VIEWPORT
	ImGuiViewport* viewport = ImGui::GetMainViewport();
	ImGui::SetNextWindowPos(viewport->GetWorkPos());
	ImGui::SetNextWindowSize(viewport->GetWorkSize());
	ImGui::SetNextWindowViewport(viewport->ID);
#else 
	ImGui::SetNextWindowPos(ImVec2(0.0f, 0.0f));
	ImGui::SetNextWindowSize(ImGui::GetIO().DisplaySize);
#endif

	ImGui::Begin("配置生成器", (bool*)true, window_flags);
	ImGui::Text("URL:"); ImGui::SameLine();
	ImGui::InputText("##URL", url, 256);
	ImGui::Text("标题:"); ImGui::SameLine();
	ImGui::InputText("##TITLE", title, 256);
	if (ImGui::Button(U8TEXT("生成")))
	{
		global.info.baseUrl = url;
		global.info.title = title;
		genconfig();
	}

	ImGui::Text("MODSync Configurer Version: 2.0");
	ImGui::Text("average %.3f ms/frame (%.1f FPS)", 1000.0f / io.Framerate, io.Framerate);
	ImGui::End();
}


int main()
{
	reloadconfig();
	GuiService gs("ModSync Configurer");
	gs.ReSize(600, 140);
	static bool done = false;
	gs.SetRenderCallBack(&render);
	while (!done)
	{
		gs.Run(&done);
	}
}

#ifdef WIN32
int WINAPI wWinMain(HINSTANCE hInstance, HINSTANCE hPrevInstance, PWSTR pCmdLine, int nCmdShow)
{
	main();
	return 0;
}
#endif // WIN32
