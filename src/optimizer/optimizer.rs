use crate::config::config::ConfigData;
use crate::llm::base::{SetupConfig, UniversalBase};
use crate::llm::llm::LLM;
use crate::llm::utils::Logger;
use crate::logger::logger::Logger as IOLogger;
use log::{debug, info};
use rand::prelude::SliceRandom;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::fmt;

#[derive(Clone, Default)]
pub struct Optimizer {
    pub config: ConfigData,
}

impl Optimizer {
    pub fn new(config: ConfigData) -> Self {
        Optimizer { config }
    }

    // TBD: FIXME
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PromptOptimizationParams {
    prompt_technique_name: String,
}

impl fmt::Display for PromptOptimizationParams {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[(\"prompt_technique_name\", \"{}\")]",
            self.prompt_technique_name
        )
    }
}

impl UniversalBase for PromptOptimizationParams {}

impl PromptOptimizationParams {
    pub fn new(prompt_technique_name: String) -> Self {
        PromptOptimizationParams {
            prompt_technique_name,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptPool {
    pub system_prompt: String,
    pub final_prompt: String,
    pub eval_prompt: String,
}

#[derive(Debug, Clone)]
pub struct DatasetSpecificProcessing {}

impl DatasetSpecificProcessing {
    const QUESTION_LITERAL: &'static str = "question";
    const ANSWER_WITH_REASON_LITERAL: &'static str = "answer";
    const FINAL_ANSWER_LITERAL: &'static str = "final_answer";
    const QUESTION_KEY_IN_PROMPT: &'static str = "[Question]";
    const ANSWER_KEY_IN_PROMPT: &'static str = "[Answer]";
    const TEXT_DELIMITER_PATTERN: &'static str = r"(?s)(?<=<START>)(.*?)(?=<END>)";
    const TEXT_DELIMITER_PATTERN_MUTATION: &'static str = r"(?s)(?<=<START>)(.*?)(?=<END>)";
    const ANSWER_START: &'static str = "<ANS_START>";
    const ANSWER_END: &'static str = "<ANS_END>";
    const ANSWER_DELIMITER_PATTERN: &'static str = r"(?s)(?<=<ANS_START>)(.*?)(?=<ANS_END>)";
    const INVALID_ANS: &'static str = "[invalid]";
    const FINAL_PROMPT: &'static str = "";

    fn normalize_prediction(&self, prediction: &str, lowercase: bool) -> String {
        let mut normalized = prediction
            .replace(" and ", " ")
            .replace("Sentence 1:", " ")
            .replace("Sentence 2:", " ")
            .trim()
            .to_string();

        if let Some(first_line) = normalized.split('\n').next() {
            normalized = first_line.to_string();
        }
        if let Some(first_sentence) = normalized.split('.').next() {
            normalized = first_sentence.to_string();
        }

        if lowercase {
            normalized = normalized.to_lowercase();
        }

        normalized = normalized
            .replace('-', " ")
            .chars()
            .filter(|c| !c.is_ascii_punctuation())
            .collect::<String>();

        normalized.trim().to_string()
    }

    fn assess_answer(&self, llm_output: &str, gt_answer: &str) -> (bool, String) {
        let predicted_answer = self.extract_final_answer(llm_output);
        let is_correct = predicted_answer.to_lowercase() == gt_answer.to_lowercase();
        (is_correct, predicted_answer)
    }

    fn collate_to_str(examples: &[HashMap<String, String>], example_template: &str) -> String {
        let mut example_string = String::new();

        for example in examples {
            let answer = if example.contains_key(Self::ANSWER_WITH_REASON_LITERAL) {
                &example[Self::ANSWER_WITH_REASON_LITERAL]
            } else {
                &example[Self::FINAL_ANSWER_LITERAL]
            };

            example_string.push_str(
                &example_template
                    .replace(
                        "{question}",
                        &example[DatasetSpecificProcessing::QUESTION_LITERAL],
                    )
                    .replace("{answer}", answer),
            );
        }

        example_string
    }

    fn extract_final_answer(&self, answer: &str) -> String {
        answer.to_string()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CritiqueNRefinePromptPool {
    pub base: PromptPool,
    pub quest_reason_ans: String,
    pub expert_profile: String,
    pub ans_delimiter_instruction: String,
    pub intent_template: String,
    pub thinking_styles: Vec<String>,
    pub meta_critique_template: String,
    pub meta_positive_critique_template: String,
    pub critique_refine_template: String,
    pub solve_template: String,
    pub examples_critique_template: String,
    pub examples_optimization_template: String,
    pub meta_sample_template: String,
    pub expert_template: String,
    pub generate_reason_template: String,
    pub reason_optimization_template: String,
    pub examples_critique_template_zero_shot: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CritiqueNRefineParams {
    pub base: PromptOptimizationParams,
    pub unique_model_id: String,
    // Number of candidate prompts to generate in given iteration
    pub style_variation: i32,
    // Number of questions to be asked to LLM in a single go
    pub questions_batch_size: i32,
    // Number of batches of questions to correctly answered, for a prompt to be considered as performing good
    pub min_correct_count: i32,
    // Max number of mini-batches on which we should evaluate our prompt
    pub max_eval_batches: i32,
    // Number of top best performing prompts to be considered for next iterations
    pub top_n: i32,
    // Number of rounds of mutation to be performed when generating different styles
    pub mutation_rounds: i32,
    // Refine instruction post mutation
    pub refine_instruction: bool,
    // Number of iterations for conducting <mutation_rounds> rounds of mutation of task description
    // followed by refinement of instructions
    pub mutate_refine_iterations: i32,
    // Number of iterations for refining task description and in context examples for few-shot
    pub refine_task_eg_iterations: i32,
    // Description of task. This will be fed to prompt
    pub task_description: String,
    // Base instruction, in line with your dataset. This will be fed to prompt
    pub base_instruction: String,
    // Instruction for specifying answer format
    pub answer_format: String,
    // Number of samples from dataset, set aside as training data. In every iteration we would be drawing
    // `questions_batch_size` examples from training data with replacement.
    pub seen_set_size: i32,
    // Number of examples to be given for few shots
    pub few_shot_count: i32,
    // Generate synthetic reasoning
    pub generate_reasoning: bool,
    // Generate description of an expert which can solve the task at hand
    pub generate_expert_identity: bool,
    // Generate keywords that describe the intent of the task
    pub generate_intent_keywords: bool,
    // number of synthetic training examples to be generated
    pub num_train_examples: i32,
}

#[derive(Debug, Clone, Copy)]
pub enum PromptScoreIndex {
    PromptStr = 0,
    Score = 1,
    Dataset = 2,
}

pub struct CritiqueNRefine {
    dataset: Vec<serde_json::Value>,
    setup_config: SetupConfig,
    data_processor: DatasetSpecificProcessing,
    logger: Logger,
    prompt_pool: CritiqueNRefinePromptPool,
    base_path: String,
    iolog: IOLogger,
}

impl CritiqueNRefine {
    pub fn new(
        dataset: Vec<serde_json::Value>,
        setup_config: SetupConfig,
        data_processor: DatasetSpecificProcessing,
        logger: Logger,
        prompt_pool: CritiqueNRefinePromptPool,
        base_path: String,
    ) -> Self {
        let mut iolog = IOLogger::new(base_path.clone()).expect("failed to create iologger");
        iolog
            .reset_eval_glue(base_path.clone())
            .expect("failed to reset eval glue");

        Self {
            dataset,
            setup_config,
            data_processor,
            logger,
            prompt_pool,
            base_path,
            iolog,
        }
    }

    pub async fn chat_completion(&self, user_prompt: &str, system_prompt: Option<&str>) -> String {
        let system_prompt = system_prompt.unwrap_or(&self.prompt_pool.base.system_prompt);

        let messages = vec![
            json!({
                "role": "system",
                "content": system_prompt
            }),
            json!({
                "role": "user",
                "content": user_prompt
            }),
        ];

        LLM.chat_completion(messages).await
    }

    pub async fn gen_different_styles(
        &self,
        base_instruction: &str,
        task_description: &str,
        mutation_rounds: usize,
        thinking_styles_count: usize,
    ) -> Vec<String> {
        let mut candidate_prompts = vec![format!("{}\n{}", task_description, base_instruction)];

        for mutation_round in 0..mutation_rounds {
            let mutated_sample_prompt = self
                .prompt_pool
                .meta_sample_template
                .replace("{task_description}", task_description)
                .replace(
                    "{meta_prompts}",
                    &self.prompt_pool.thinking_styles[..thinking_styles_count].join("\n"),
                )
                .replace("{num_variations}", &thinking_styles_count.to_string())
                .replace("{prompt_instruction}", base_instruction);

            let generated_mutated_prompt = self.chat_completion(&mutated_sample_prompt, None).await;

            let re = Regex::new(&self.data_processor.text_delimiter_pattern).unwrap();
            let matches: Vec<String> = re
                .find_iter(&generated_mutated_prompt)
                .map(|m| m.as_str().to_string())
                .collect();

            candidate_prompts.extend(matches);

            info!(
                "{}",
                &format!(
                    "mutation_round={} mutated_sample_prompt={} mutated_prompt_generation={}",
                    mutation_round, mutated_sample_prompt, generated_mutated_prompt
                )
            );
        }

        candidate_prompts
    }

    pub async fn critique_and_refine(
        &self,
        prompt: &str,
        critique_example_set: &[serde_json::Value],
        further_enhance: bool,
    ) -> String {
        let example_string = self
            .data_processor
            .collate_to_str(critique_example_set, &self.prompt_pool.quest_reason_ans);

        let meta_critique_prompt = if further_enhance {
            &self.prompt_pool.meta_positive_critique_template
        } else {
            &self.prompt_pool.meta_critique_template
        };

        let meta_critique_prompt = meta_critique_prompt
            .replace("{instruction}", prompt)
            .replace("{examples}", &example_string);

        let critique_text = self
            .chat_completion(
                &meta_critique_prompt,
                Some(&self.prompt_pool.expert_profile),
            )
            .await;

        let critique_refine_prompt = self
            .prompt_pool
            .critique_refine_template
            .replace("{instruction}", prompt)
            .replace("{examples}", &example_string)
            .replace("{critique}", &critique_text)
            .replace("{steps_per_sample}", "1");

        let refined_prompts = self
            .chat_completion(
                &critique_refine_prompt,
                Some(&self.prompt_pool.expert_profile),
            )
            .await;

        if let Ok(re) = Regex::new(&self.data_processor.text_delimiter_pattern.as_str()) {
            if let Some(caps) = re.captures(&refined_prompts) {
                info!(
                    "{}",
                    &format!(
                        "Prompt to get critique: {}\n\
                    Critique received from LLM: {}\n\
                    Prompt to get Refinement after critique, from LLM: {}\n\
                    Refined prompts received from LLM: {}",
                        meta_critique_prompt,
                        critique_text,
                        critique_refine_prompt,
                        caps[1].to_string()
                    )
                );
                return Ok(caps[1].to_string());
            }
        }

        Err("The LLM output is not in the expected format. Please rerun the code...".into())
    }

    fn get_prompt_score(
        &self,
        instructions: Vec<String>,
        params: PromptOptimizationParams,
    ) -> Vec<(String, f64, Vec<HashMap<String, String>>)> {
        let mut prompt_score_list = Vec::new();
        let mut rng = rand::thread_rng();

        for instruction in instructions {
            let mut correct_count = 0.0;
            let mut count = 0.0;
            let mut critique_example_set = Vec::new();

            let mut dataset_subset: Vec<_> = self
                .dataset
                .choose_multiple(&mut rng, params.questions_batch_size)
                .collect();
            let mut questions_pool: Vec<_> = dataset_subset
                .iter()
                .map(|example| {
                    example
                        .get(DatasetSpecificProcessing::QUESTION_LITERAL)
                        .unwrap()
                })
                .collect();

            while critique_example_set.is_empty()
                && correct_count < params.min_correct_count as f64
                && count < params.max_eval_batches as f64
            {
                count += 1.0;
                let solve_prompt = self
                    .prompt_pool
                    .solve_template
                    .replace(
                        "{questions_batch_size}",
                        &params.questions_batch_size.to_string(),
                    )
                    .replace("{answer_format}", &params.answer_format)
                    .replace("{instruction}", &instruction)
                    .replace("{questions}", &questions_pool.join("\n"));

                let generated_text = self.chat_completion(&solve_prompt);
                critique_example_set = self.evaluate(&generated_text, &dataset_subset);

                if critique_example_set.is_empty() {
                    dataset_subset = self
                        .dataset
                        .choose_multiple(&mut rng, params.questions_batch_size)
                        .collect();
                    questions_pool = dataset_subset
                        .iter()
                        .map(|example| {
                            example
                                .get(DatasetSpecificProcessing::QUESTION_LITERAL)
                                .unwrap()
                        })
                        .collect();
                    correct_count += 1.0;
                }

                println!("critique_example_set: {:?}", critique_example_set);
                println!("correct_count: {}", correct_count);
            }
            println!("Loop completed");
            prompt_score_list.push((instruction, correct_count / count, dataset_subset));
        }

        info!("prompt_score_list {:?}", prompt_score_list);

        prompt_score_list
    }

    pub fn refine_prompts(
        &self,
        prompt_score_list: Vec<(String, f64, Vec<Example>)>,
        params: &PromptOptimizationParams,
    ) -> Vec<String> {
        let mut refined_prompts = Vec::new();

        for (prompt, score, critique_example_set) in prompt_score_list {
            let threshold = params.min_correct_count / params.max_eval_batches as f64;
            if score >= threshold {
                // Good prompt refinement
                refined_prompts.push(self.critique_and_refine(
                    &prompt,
                    &critique_example_set,
                    true,
                ));
            } else {
                // Not good enough prompt refinement
                refined_prompts.push(self.critique_and_refine(
                    &prompt,
                    &critique_example_set,
                    false,
                ));
            }
        }

        info!("refined_prompts: {:?}", refined_prompts);

        refined_prompts
    }

    pub fn evaluate(
        &self,
        generated_text: &str,
        dataset_subset: &[HashMap<String, String>],
    ) -> Vec<HashMap<String, String>> {
        let re = Regex::new(DatasetSpecificProcessing::ANSWER_DELIMITER_PATTERN).unwrap();
        let mut answer_matches: Vec<&str> =
            re.find_iter(generated_text).map(|m| m.as_str()).collect();

        let answers_len = answer_matches.len();
        let dataset_len = dataset_subset.len();

        if answers_len != dataset_len {
            info!(
                "Answers extracted from LLM output={}, Questions asked to LLM {}",
                answers_len, dataset_len,
            );
            if answers_len > dataset_len {
                answer_matches.truncate(dataset_len);
            }
        }

        let mut wrong_examples = Vec::new();
        for i in 0..std::cmp::min(answers_len, dataset_len) {
            println!("dataset_subset: {:?}", dataset_subset);
            let actual_answer = &dataset_subset[i][DatasetSpecificProcessing::FINAL_ANSWER_LITERAL];
            let question = &dataset_subset[i][DatasetSpecificProcessing::QUESTION_LITERAL];
            let (is_correct, _) = self
                .data_processor
                .access_answer(answer_matches[i], actual_answer);
            if !is_correct {
                wrong_examples.push(dataset_subset[i].clone());
            }
        }

        wrong_examples
    }

    pub fn select_top_prompts(
        &self,
        prompt_score_list: Vec<(String, f64, Vec<Example>)>,
        top_n: usize,
    ) -> Vec<(String, f64, Vec<Example>)> {
        let mut sorted_prompts = prompt_score_list;
        sorted_prompts.sort_by(|a, b| {
            b.1.partial_cmp(&a.1)
                .unwrap()
                .then_with(|| b.0.len().cmp(&a.0.len()))
        });

        let sorted_top_n_prompts = sorted_prompts.into_iter().take(top_n).collect();

        debug!("Sorted top n prompts: {:?}", sorted_top_n_prompts);

        sorted_top_n_prompts
    }

    pub fn extract_examples_from_response(
        &self,
        response_with_examples: &str,
    ) -> Vec<HashMap<String, String>> {
        let mut synthetic_examples = Vec::new();
        let re = Regex::new(DatasetSpecificProcessing::TEXT_DELIMITER_PATTERN).unwrap();

        for cap in re.find_iter(response_with_examples) {
            let text = cap.as_str().trim();

            if text.contains(DatasetSpecificProcessing::QUESTION_KEY_IN_PROMPT)
                && text.contains(DatasetSpecificProcessing::ANSWER_KEY_IN_PROMPT)
            {
                let question = self
                    .extract_between(
                        DatasetSpecificProcessing::QUESTION_KEY_IN_PROMPT,
                        DatasetSpecificProcessing::ANSWER_KEY_IN_PROMPT,
                        text,
                    )
                    .trim()
                    .to_string();

                let answer_with_reason = text[text
                    .find(DatasetSpecificProcessing::ANSWER_KEY_IN_PROMPT)
                    .map(|i| i + DatasetSpecificProcessing::ANSWER_KEY_IN_PROMPT.len())
                    .unwrap_or(0)..]
                    .trim()
                    .to_string();

                let final_answer = match &self.data_processor {
                    Some(processor) => processor.extract_final_answer(&answer_with_reason),
                    None => self.extract_between(
                        DatasetSpecificProcessing::ANSWER_START,
                        DatasetSpecificProcessing::ANSWER_END,
                        &answer_with_reason,
                    ),
                };

                let mut formatted_data = HashMap::new();
                formatted_data.insert(
                    DatasetSpecificProcessing::QUESTION_LITERAL.to_string(),
                    question,
                );
                formatted_data.insert(
                    DatasetSpecificProcessing::ANSWER_WITH_REASON_LITERAL.to_string(),
                    answer_with_reason,
                );
                formatted_data.insert(
                    DatasetSpecificProcessing::FINAL_ANSWER_LITERAL.to_string(),
                    final_answer,
                );

                synthetic_examples.push(formatted_data);
            }
        }

        synthetic_examples
    }

    pub fn generate_reasoning(
        &self,
        task_description: &str,
        instruction: &str,
        question: &str,
        answer: &str,
    ) -> String {
        let prompt_template = self
            .prompt_pool
            .generate_reason_template
            .replace("{task_description}", task_description)
            .replace("{instruction}", instruction)
            .replace("{question}", question)
            .replace("{answer}", answer);

        self.llm_mgr.chat_completion(&prompt_template)
    }

    pub fn generate_expert_identity(&self, task_description: &str) -> String {
        let expert_prompt = self
            .prompt_pool
            .expert_template
            .replace("{task_description}", task_description);

        self.llm_mgr.chat_completion(&expert_prompt)
    }

    pub fn generate_intent_keywords(&self, task_description: &str, instruction: &str) -> String {
        let prompt_template = self
            .prompt_pool
            .intent_template
            .replace("{task_description}", task_description)
            .replace("{instruction}", instruction);

        self.llm_mgr.chat_completion(&prompt_template)
    }

    pub fn generate_best_examples(
        &self,
        examples: &[HashMap<String, String>],
        params: &CritiqueNRefineParams,
    ) -> Vec<HashMap<String, String>> {
        let example_string = self
            .data_processor
            .collate_to_str(examples, &self.prompt_pool.quest_reason_ans);

        let few_shot_critique_prompt = self
            .prompt_pool
            .examples_critique_template
            .replace("{prompt}", &params.base_instruction)
            .replace("{examples}", &example_string)
            .replace("{task_description}", &params.task_description)
            .replace("{num_examples}", &params.few_shot_count.to_string());

        let critique = self.llm_mgr.chat_completion_with_system(
            &few_shot_critique_prompt,
            &self.prompt_pool.expert_profile,
        );

        let gt_eg = self
            .dataset
            .choose(&mut rand::thread_rng())
            .expect("Dataset should not be empty");
        let gt_eg_string = self
            .data_processor
            .collate_to_str(&[gt_eg.clone()], &self.prompt_pool.quest_reason_ans);

        let few_shot_opt_prompt = self
            .prompt_pool
            .examples_optimization_template
            .replace("{prompt}", &params.base_instruction)
            .replace("{examples}", &example_string)
            .replace("{gt_example}", &gt_eg_string)
            .replace("{critique}", &critique)
            .replace("{task_description}", &params.task_description)
            .replace("{num_examples}", &params.few_shot_count.to_string());

        let synthetic_examples = self
            .llm
            .chat_completion(&few_shot_opt_prompt, &self.prompt_pool.expert_profile);

        self.extract_examples_from_response(&synthetic_examples)
    }

    pub async fn generate_best_examples_zero_shot(
        &self,
        params: &CritiqueNRefineParams,
    ) -> Vec<Example> {
        let few_shot_critique_prompt = self
            .prompt_pool
            .examples_critique_template_zero_shot
            .replace("{prompt}", &params.base_instruction)
            .replace("{task_description}", &params.task_description)
            .replace("{num_examples}", &params.num_train_examples.to_string());

        let critique = self
            .chat_completion(
                &few_shot_critique_prompt,
                self.prompt_pool.expert_profile.as_deref(),
            )
            .await;

        let few_shot_opt_prompt = self
            .prompt_pool
            .examples_optimization_template
            .replace("{prompt}", &params.base_instruction)
            .replace("{examples}", "")
            .replace("{gt_example}", "")
            .replace("{critique}", &critique)
            .replace("{task_description}", &params.task_description)
            .replace("{num_examples}", &params.num_train_examples.to_string());

        let synthetic_examples = self
            .chat_completion(
                &few_shot_opt_prompt,
                self.prompt_pool.expert_profile.as_deref(),
            )
            .await;

        self.extract_examples_from_response(&synthetic_examples)
    }

    pub async fn get_best_instr_by_critique(
        &self,
        examples: Vec<HashMap<String, String>>,
        params: &CritiqueNRefineParams,
    ) -> Option<String> {
        let example_string = if let Some(ref data_processor) = self.data_processor {
            // Assuming collate_to_str is a method of DataProcessor
            data_processor.collate_to_str(&examples, &self.prompt_pool.quest_reason_ans)
        } else {
            examples
                .iter()
                .map(|example| {
                    let answer = example
                        .get(DatasetSpecificProcessing::FINAL_ANSWER_LITERAL)
                        .or_else(|| {
                            example.get(DatasetSpecificProcessing::ANSWER_WITH_REASON_LITERAL)
                        })
                        .unwrap_or(&DatasetSpecificProcessing::INVALID_ANS.to_string());
                    format!(
                        self.prompt_pool.quest_reason_ans,
                        question = example
                            .get(DatasetSpecificProcessing::QUESTION_LITERAL)
                            .unwrap_or(&String::new()),
                        answer = answer
                    )
                })
                .collect::<String>()
        };

        let meta_critique_prompt = format!(
            self.prompt_pool.meta_critique_template,
            instruction = params.base_instruction,
            examples = example_string
        );
        let critique_text =
            self.chat_completion(&meta_critique_prompt, &self.prompt_pool.expert_profile);
        let critique_refine_prompt = format!(
            self.prompt_pool.critique_refine_template,
            instruction = params.base_instruction,
            examples = example_string,
            critique = critique_text,
            steps_per_sample = 1
        );
        let refined_prompts = self.chat_completion(&critique_refine_prompt);

        let refined_instructions = if let Some(ref data_processor) = self.data_processor {
            data_processor
                .TEXT_DELIMITER_PATTERN
                .find_iter(&refined_prompts)
                .map(|mat| mat.as_str().to_string())
                .collect::<Vec<String>>()
        } else {
            Regex::new(DatasetSpecificProcessing::TEXT_DELIMITER_PATTERN)
                .unwrap()
                .find_iter(&refined_prompts)
                .map(|mat| mat.as_str().to_string())
                .collect::<Vec<String>>()
        };

        refined_instructions.get(0).cloned()
    }

    pub async fn get_best_prompt(
        &mut self,
        params: &mut CritiqueNRefineParams,
        use_examples: bool,
        run_without_train_examples: bool,
        generate_synthetic_examples: bool,
    ) -> (String, String) {
        let mut current_base_instruction = params.base_instruction.clone();

        if !generate_synthetic_examples {
            println!("\nMutating Task Description....");
            for round_num in 1..=params.mutate_refine_iterations {
                info!(&format!(
                    "{} + Starting iteration: {} \n current_base_instruction: {}",
                    CommonLogsStr::LOG_SEPERATOR,
                    round_num,
                    current_base_instruction
                ));
                let candidate_prompts = self.gen_different_styles(
                    &current_base_instruction,
                    &params.task_description,
                    params.mutation_rounds + 1,
                    params.style_variation,
                );

                if run_without_train_examples {
                    let mut prompt_index = 1;
                    println!("\nOptimization Finished...");
                    println!("\nPossible prompt variations:");
                    for candidate in &candidate_prompts[..params.mutation_rounds] {
                        let final_best_prompt = self
                            .prompt_pool
                            .base
                            .final_prompt
                            .replace("{instruction}", candidate)
                            .replace("{answer_format}", &params.answer_format)
                            .replace("{few_shot_examples}", "");
                        let mut expert_identity = self.prompt_pool.base.system_prompt.clone();
                        if params.generate_expert_identity {
                            expert_identity =
                                self.generate_expert_identity(&params.task_description);
                        }
                        let intent_keywords = self.generate_intent_keywords(
                            &params.task_description,
                            &params.base_instruction,
                        );
                        let final_best_prompt =
                            format!("{}Keywords: {}", final_best_prompt, intent_keywords);
                        println!("_______________________________________________________________________");
                        println!(
                            "\nVariations {}:\nExpert Profile:\n{}:\nPrompt:\n{}",
                            prompt_index, expert_identity, final_best_prompt
                        );
                        prompt_index += 1;
                    }
                    return ("".to_string(), "".to_string());
                }
                let mut prompt_score_list = self.get_prompt_score(&candidate_prompts, params);
                prompt_score_list = self.select_top_prompts(&prompt_score_list, params.top_n);

                if params.refine_instruction {
                    let refined_prompts = self.refine_prompts(&prompt_score_list, params);
                    let refined_prompt_score_list = self.get_prompt_score(&refined_prompts, params);
                    prompt_score_list = self.select_top_prompts(
                        &(refined_prompt_score_list + prompt_score_list),
                        params.top_n,
                    );
                }

                current_base_instruction =
                    prompt_score_list[0][self.GetPromptScoreIndex.PROMPT_STR].clone();
                self.iolog.append_dict_to_chained_logs(&json!({"round_num": round_num, "best_prompt": current_base_instruction, "score": prompt_score_list[0][self.GetPromptScoreIndex.SCORE]}));
            }

            let mut examples = Vec::new();
            params.base_instruction = current_base_instruction.clone();
            for example in &self.dataset {
                let solve_prompt = self
                    .prompt_pool
                    .solve_template
                    .replace("{questions_batch_size}", "1")
                    .replace("{instruction}", &params.base_instruction)
                    .replace("{answer_format}", &params.answer_format)
                    .replace(
                        "{questions}",
                        &example[DatasetSpecificProcessing::QUESTION_LITERAL],
                    );
                let generated_text = self.chat_completion(&solve_prompt);
                examples.extend(self.evaluate(&generated_text, &[example]));
                if examples.len() >= params.few_shot_count {
                    break;
                }
            }

            if examples.len() < params.few_shot_count {
                examples.extend(self.dataset.choose_multiple(
                    &mut rand::thread_rng(),
                    params.few_shot_count - examples.len(),
                ));
            }

            println!("\nRefining Task description and Examples iteratively....");
            for _ in 0..params.refine_task_eg_iterations {
                let refine_task_desc = rand::random::<bool>();
                if refine_task_desc {
                    if let Some(refined_instruction) =
                        self.get_best_instr_by_critique(&examples, params).await
                    {
                        params.base_instruction = refined_instruction;
                    }
                } else if use_examples {
                    examples = self.generate_best_examples(&examples, params);
                }
            }
        } else {
            println!("Generating Synthetic Examples....");
            let train_examples = self.generate_best_examples_zero_shot(params);
            let mut file = File::create("train_synthetic.jsonl").unwrap();
            for record in train_examples {
                writeln!(file, "{}", json!(record)).unwrap();
            }
            println!("Synthetic examples saved at train.jsonl....");
            return ("".to_string(), "".to_string());
        }

        if params.generate_reasoning {
            println!("\nGenerating CoT Reasoning for In-Context Examples....");
            for example in &mut examples {
                let reason = self.generate_reasoning(
                    &params.task_description,
                    &params.base_instruction,
                    &example[DatasetSpecificProcessing::QUESTION_LITERAL],
                    &example[DatasetSpecificProcessing::FINAL_ANSWER_LITERAL],
                );
                example.insert(
                    DatasetSpecificProcessing::ANSWER_WITH_REASON_LITERAL.to_string(),
                    format!(
                        "{} {}{}{}",
                        reason,
                        DatasetSpecificProcessing::ANSWER_START,
                        example[DatasetSpecificProcessing::FINAL_ANSWER_LITERAL],
                        DatasetSpecificProcessing::ANSWER_END
                    ),
                );
            }
        }

        let example_string = if let Some(data_processor) = &self.data_processor {
            data_processor.collate_to_str(&examples, &self.prompt_pool.quest_reason_ans)
        } else {
            examples
                .iter()
                .map(|example| {
                    let answer = example
                        .get(DatasetSpecificProcessing::ANSWER_WITH_REASON_LITERAL)
                        .unwrap_or(&example[DatasetSpecificProcessing::FINAL_ANSWER_LITERAL]);
                    self.prompt_pool
                        .quest_reason_ans
                        .replace(
                            "{question}",
                            &example[DatasetSpecificProcessing::QUESTION_LITERAL],
                        )
                        .replace("{answer}", answer)
                })
                .collect::<Vec<_>>()
                .join("")
        };

        let mut final_best_prompt = if params.few_shot_count == 0 {
            self.prompt_pool
                .base
                .final_prompt
                .replace("{instruction}", &params.base_instruction)
                .replace("{answer_format}", &params.answer_format)
                .replace("{few_shot_examples}", "")
        } else {
            self.prompt_pool
                .base
                .final_prompt
                .replace("{instruction}", &params.base_instruction)
                .replace("{answer_format}", &params.answer_format)
                .replace("{few_shot_examples}", &example_string)
        };

        let mut expert_identity = self.prompt_pool.base.system_prompt.clone();
        if params.generate_expert_identity {
            println!("\nGenerating Expert Identity....");
            expert_identity = self.generate_expert_identity(&params.task_description);
            info!("{}", &format!("Expert Identity: {}", expert_identity));
        }

        if params.generate_intent_keywords {
            println!("\nGenerating Intent Keywords....");
            let intent_keywords =
                self.generate_intent_keywords(&params.task_description, &params.base_instruction);
            final_best_prompt.push_str(&format!("Keywords: {}", intent_keywords));
        }

        self.iolog
            .dump_chained_log_to_file("best_prompt")
            .expect("failed to dump chained log");
        info!("{}", &format!("Final best prompt: {}", final_best_prompt));

        (final_best_prompt, expert_identity)
    }

    fn extract_between(start: &str, end: &str, text: &str) -> String {
        let start_idx = match text.find(start) {
            Some(idx) => idx + start.len(),
            None => return String::new(),
        };

        let end_idx = match text[start_idx..].find(end) {
            Some(idx) => start_idx + idx,
            None => return String::new(),
        };

        text[start_idx..end_idx].to_string()
    }
}
