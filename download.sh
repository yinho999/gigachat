# Requires git lfs
# Variable name - $LLM_MODELS
LLM_MODELS_PATH="TheBloke/Wizard-Vicuna-7B-Uncensored-GGML"
git lfs install
echo "Cloning $LLM_MODELS_PATH"
sudo git clone https://huggingface.co/$LLM_MODELS_PATH /home/"$USER"/.llm-models/"$LLM_MODELS_PATH"