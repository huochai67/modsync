#include <stdio.h>          // printf, fprintf
#include <cstdlib>
#include <functional>
#include <string>

#include <imgui.h>
#include <imgui_impl_sdl2.h>
#include <imgui_impl_vulkan.h>

#define SDL_MAIN_HANDLED
#include <SDL2/SDL.h>
#include <SDL2/SDL_vulkan.h>
#include <vulkan/vulkan.h>

#ifdef _DEBUG
#define IMGUI_VULKAN_DEBUG_REPORT
#endif // _DEBUG

typedef std::function<void()> RenderCallBack;
class GuiService
{
public:
	~GuiService();
	GuiService(const char* title_ = "Title");

	void Run(bool* done);
	void ReSize(const size_t& x, const size_t& w);
	void SetTitle(const char* title_);
	void SetTitle(const std::string& title_) { this->SetTitle(title_.c_str()); };

	inline void SetRenderCallBack(RenderCallBack callback_) { this->m_callback = callback_; };
private:
	RenderCallBack m_callback = nullptr;

	void SetupVulkan(const char** extensions, uint32_t extensions_count);
	void SetupVulkanWindow(ImGui_ImplVulkanH_Window* wd, VkSurfaceKHR surface, int width, int height);
	void CleanupVulkan();
	void CleanupVulkanWindow();

	void FrameRender(ImGui_ImplVulkanH_Window* wd, ImDrawData* draw_data);
	void FramePresent(ImGui_ImplVulkanH_Window* wd);

	VkAllocationCallbacks* g_Allocator = NULL;
	VkInstance               g_Instance = VK_NULL_HANDLE;
	VkPhysicalDevice         g_PhysicalDevice = VK_NULL_HANDLE;
	VkDevice                 g_Device = VK_NULL_HANDLE;
	uint32_t                 g_QueueFamily = (uint32_t)-1;
	VkQueue                  g_Queue = VK_NULL_HANDLE;
	VkDebugReportCallbackEXT g_DebugReport = VK_NULL_HANDLE;
	VkPipelineCache          g_PipelineCache = VK_NULL_HANDLE;
	VkDescriptorPool         g_DescriptorPool = VK_NULL_HANDLE;
	SDL_Window* m_window;

	ImGui_ImplVulkanH_Window g_MainWindowData;
	uint32_t                 g_MinImageCount = 2;
	bool                     g_SwapChainRebuild = false;

	ImVec4 clear_color = ImVec4(0.45f, 0.55f, 0.60f, 1.00f);
};
